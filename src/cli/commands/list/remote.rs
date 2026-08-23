#[derive(usage::Args)]
pub struct Args {}

#[derive(serde::Deserialize)]
pub struct GithubRelease {
    pub tag_name: String,
}

fn fetch_gql_releases(
    github_config: &crate::cli::config::GithubConfig,
) -> Vec<String> {
    let client = reqwest::blocking::Client::new();
    let request = client
        .get(format!(
            "https://api.github.com/repos/{}/{}/releases",
            github_config.owner, github_config.repository
        ))
        .header("User-Agent", "gqlup")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2026-03-10")
        .build()
        .unwrap();
    let result: Vec<GithubRelease> =
        serde_json::from_reader(client.execute(request).unwrap()).unwrap();
    return result.into_iter().map(|r| r.tag_name).collect();
}

fn run(_: Args) {
    let config = crate::cli::config::load_config_or_exit();
    for version in fetch_gql_releases(&config.github) {
        println!("{version}");
    }
}

impl usage::Run for Args {
    type Output = ();

    fn run(self) -> Self::Output {
        run(self);
        ()
    }
}
