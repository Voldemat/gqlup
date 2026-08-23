#[derive(usage::Args)]
pub struct Args {
    pub version: String,
}

#[derive(serde::Deserialize)]
pub struct GithubAsset {
    pub name: String,
    pub id: u64,
}

#[derive(serde::Deserialize)]
pub struct GithubReleaseTagsResponse {
    pub assets: Vec<GithubAsset>,
}

const GQL_OS_NAME: &'static str = cfg_select! {
    target_os = "linux" => "linux",
    target_os = "macos" => "darwin",
    _ => compiler_error!("Unsupported operating system!"),
};
const GQL_ARCH_NAME: &'static str = cfg_select! {
    target_arch = "x86_64" => "x86_64",
    target_arch = "aarch64" => "arm64",
    _ => compiler_error!("Unsupported architecture!"),
};

#[derive(serde::Deserialize)]
struct GithubReleaseResponse {
    pub tag_name: String,
}

fn fetch_latest_release_version(
    github_config: &crate::cli::config::GithubConfig,
) -> semver::Version {
    let client = reqwest::blocking::Client::new();
    let request = client
        .get(format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            github_config.owner, github_config.repository
        ))
        .header("User-Agent", "gqlup")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2026-03-10")
        .build()
        .unwrap();
    let result: GithubReleaseResponse =
        serde_json::from_reader(client.execute(request).unwrap()).unwrap();
    semver::Version::parse(&result.tag_name).unwrap()
}

fn fetch_gql_tag_gz(
    github_config: &crate::cli::config::GithubConfig,
    version: &str,
) -> impl std::io::Read {
    let client = reqwest::blocking::Client::new();
    let request = client
        .get(format!(
            "https://api.github.com/repos/{}/{}/releases/tags/{}",
            github_config.owner, github_config.repository, version
        ))
        .header("User-Agent", "gqlup")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2026-03-10")
        .build()
        .unwrap();
    let result: GithubReleaseTagsResponse =
        serde_json::from_reader(client.execute(request).unwrap()).unwrap();
    let asset_name = format!("gql-{GQL_OS_NAME}-{GQL_ARCH_NAME}.tar.gz");
    let Some(asset_id) = result
        .assets
        .into_iter()
        .find(|asset| asset.name == asset_name)
        .map(|asset| asset.id)
    else {
        eprintln!(
            "Failed to find asset with name: {asset_name} for release version {version}"
        );
        std::process::exit(1);
    };
    client
        .execute(
            client
                .get(format!(
                    "https://api.github.com/repos/{}/{}/releases/assets/{}",
                    github_config.owner, github_config.repository, asset_id
                ))
                .header("User-Agent", "gqlup")
                .header("Accept", "application/octet-stream")
                .header("X-GitHub-Api-Version", "2026-03-10")
                .build()
                .unwrap(),
        )
        .unwrap()
}

const GQL_BASH_REDIRECT_CONTENT: &'static str = r#"#!/bin/sh
exec gqlup exec "$@"
"#;

fn install_gql_bash_redirect() {
    let gql_binary_path = crate::cli::paths::get_gql_binary_path();
    if std::fs::exists(&gql_binary_path).unwrap() {
        return;
    };
    let mut target_file = std::os::unix::fs::OpenOptionsExt::mode(
        std::fs::OpenOptions::new().create(true).write(true),
        0o500,
    )
    .open(gql_binary_path)
    .unwrap();
    std::io::Write::write(
        &mut target_file,
        GQL_BASH_REDIRECT_CONTENT.as_bytes(),
    )
    .unwrap();
}

fn run(args: Args) {
    let gql_binaries_path = crate::cli::paths::get_gqlup_binary_install_path();
    if !std::fs::exists(&gql_binaries_path).unwrap() {
        eprintln!("Gqlup binaries path {gql_binaries_path:#?} does not exist");
        std::process::exit(1);
    }
    let gql_version_path =
        gql_binaries_path.join(format!("gql-{}", args.version));
    let config = crate::cli::config::load_config_or_exit();
    let version = if args.version == "latest" {
        let local_latest_version =
            crate::cli::shared::resolve_current_latest_version(
                &gql_version_path,
            );
        let remote_latest_version =
            fetch_latest_release_version(&config.github);
        if local_latest_version
            .map(|l| l == remote_latest_version)
            .unwrap_or(false)
        {
            println!(
                "Version latest ({remote_latest_version}) is already installed"
            );
            return;
        }
        remote_latest_version.to_string()
    } else {
        if std::fs::exists(&gql_version_path).unwrap() {
            println!("Version {} is already installed", args.version);
            return;
        }
        args.version
    };
    let archive_stream = fetch_gql_tag_gz(&config.github, &version);
    let decompressed = flate2::read::GzDecoder::new(archive_stream);
    let mut archive = tar::Archive::new(decompressed);
    let asset_name = format!("./gql-{GQL_OS_NAME}-{GQL_ARCH_NAME}");
    let mut gql_binary_stream = archive
        .entries()
        .unwrap()
        .into_iter()
        .map(|entry| entry.unwrap())
        .find(|entry| {
            let path = entry.path().unwrap();
            *path == *asset_name
        })
        .expect(&format!(
            "Failed to find {} asset in tar.gz archive",
            asset_name
        ));
    let target_file_path = gql_binaries_path.join(format!("gql-{}", version));
    let mut target_file = std::os::unix::fs::OpenOptionsExt::mode(
        std::fs::OpenOptions::new().create(true).write(true),
        0o500,
    )
    .open(gql_binaries_path.join(&target_file_path))
    .unwrap();
    std::io::copy(&mut gql_binary_stream, &mut target_file).unwrap();
    if config.default_version == "latest" {
        let gql_latest_path = gql_binaries_path.join("gql-latest");
        let current_version =
            crate::cli::shared::resolve_current_latest_version(
                &gql_latest_path,
            );
        if current_version
            .map(|v| v < semver::Version::parse(&version).unwrap())
            .unwrap_or(true)
        {
            std::os::unix::fs::symlink(target_file_path, gql_latest_path)
                .unwrap()
        }
    }
    install_gql_bash_redirect()
}

impl usage::Run for Args {
    type Output = ();

    fn run(self) -> Self::Output {
        run(self);
        ()
    }
}
