#[derive(usage::Args)]
pub struct Args {
    pub version: String,
    #[usage(long, default = "false")]
    pub remap_latest: bool,
}

fn run(args: Args) {
    let gql_binaries_path = crate::cli::paths::get_gqlup_binary_install_path();
    if !std::fs::exists(&gql_binaries_path).unwrap() {
        eprintln!("Gqlup binaries path {gql_binaries_path:#?} does not exist");
        std::process::exit(1);
    }
    let gql_version_path =
        gql_binaries_path.join(format!("gql-{}", args.version));
    if !std::fs::exists(&gql_version_path).unwrap() {
        eprintln!("Version {} is not installed", args.version);
        std::process::exit(1);
    }
    let config = crate::cli::config::load_config_or_exit();
    if args.version != "latest" {
        if config.default_version == args.version {
            eprintln!(
                "Version {} is set as default, cannot uninstall. Set default version to different version first.",
                args.version
            );
            std::process::exit(1);
        }
        std::fs::remove_file(gql_version_path).unwrap();
    } else {
        let gql_latest_path = gql_binaries_path.join("gql-latest");
        let Some(current_version) =
            crate::cli::shared::resolve_current_latest_version(
                &gql_latest_path,
            )
        else {
            eprintln!("Version latest is not installed");
            std::process::exit(1);
        };
        if config.default_version == "latest" && !args.remap_latest {
            eprintln!(
                "Version latest is set as default, and --remap-latest=false. Add a flag --remap-latest=true to delete current latest version and remap to most recent previous one."
            );
            std::process::exit(1);
        }
        if config.default_version == current_version.to_string() {
            eprintln!(
                "Current latest version {} is set as default, Set default version to different version first.",
                config.default_version
            );
            std::process::exit(1);
        }
        std::fs::remove_file(&gql_latest_path).unwrap();
        std::fs::remove_file(
            gql_binaries_path.join(format!("gql-{}", current_version)),
        )
        .unwrap();
        let available_versions =
            crate::cli::shared::resolve_local_versions(&gql_binaries_path);
        if available_versions.len() == 0 {
            return;
        } else {
            std::os::unix::fs::symlink(
                gql_binaries_path.join(format!(
                    "gql-{}",
                    available_versions.first().unwrap()
                )),
                gql_latest_path,
            )
            .unwrap()
        }
    }
}

impl usage::Run for Args {
    type Output = ();

    fn run(self) -> Self::Output {
        run(self);
        ()
    }
}
