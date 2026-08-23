#[derive(serde::Deserialize)]
pub struct GQLConfig {
    pub version: String,
}

pub fn exec(mut argv: Vec<std::ffi::OsString>) {
    argv.remove(0);
    let version = if std::fs::exists("./gql.yaml").unwrap() {
        let gql_config: GQLConfig =
            serde_yaml::from_reader(std::fs::File::open("./gql.yaml").unwrap())
                .unwrap();
        gql_config.version
    } else {
        let gqlup_config = crate::cli::config::load_config_or_exit();
        gqlup_config.default_version
    };
    let gql_path = crate::cli::paths::get_gqlup_binary_install_path()
        .join(format!("gql-{version}"));
    if !std::fs::exists(&gql_path).unwrap() {
        eprintln!(
            "Gql version {version} is not installed, consider installing: \"gqlup install {version}\""
        );
        std::process::exit(1);
    }
    panic!(
        "{}",
        std::os::unix::process::CommandExt::exec(
            std::process::Command::new(gql_path).args(argv)
        )
    );
}
