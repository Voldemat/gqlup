#[derive(serde::Deserialize)]
pub struct GithubConfig {
    pub owner: String,
    pub repository: String,
}

#[derive(serde::Deserialize)]
pub struct BookkeepingConfig {
    pub max_versions_count: u8,
    #[serde(with = "humantime_serde")]
    pub max_version_age: std::time::Duration,
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub github: GithubConfig,
    pub bookkeeping: BookkeepingConfig,
    pub default_version: String,
}

pub fn load_config_or_exit() -> Config {
    serde_yaml::from_reader(
        std::fs::File::open(super::paths::get_gqlup_config_path()).unwrap(),
    )
    .unwrap()
}
