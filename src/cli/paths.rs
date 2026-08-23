fn get_env_as_pathbuf(
    name: &str,
) -> Result<std::path::PathBuf, std::env::VarError> {
    std::env::var(name).map(|var| std::path::PathBuf::from(var))
}

fn get_env_or_exit(name: &str) -> std::path::PathBuf {
    get_env_as_pathbuf(name).unwrap_or_else(|error| {
        eprintln!("Failed to get {name} env: {error}");
        std::process::exit(1);
    })
}

fn get_xdg_bin_home() -> std::path::PathBuf {
    get_env_or_exit("XDG_BIN_HOME")
}

pub fn get_gql_binary_path() -> std::path::PathBuf {
    get_env_as_pathbuf("GQLUP_GQL_BIN_PATH")
        .unwrap_or_else(|_| get_xdg_bin_home().join("gql"))
}

fn get_xdg_config_home() -> std::path::PathBuf {
    get_env_or_exit("XDG_CONFIG_HOME")
}

pub fn get_gqlup_config_path() -> std::path::PathBuf {
    get_env_as_pathbuf("GQLUP_CONFIG_PATH").unwrap_or_else(|_| {
        get_xdg_config_home().join("gqlup").join("config.yaml")
    })
}

fn get_xdg_state_home() -> std::path::PathBuf {
    get_env_or_exit("XDG_STATE_HOME")
}

pub fn get_gqlup_binary_install_path() -> std::path::PathBuf {
    get_env_as_pathbuf("GQLUP_BIN_INSTALL_PATH")
        .unwrap_or_else(|_| get_xdg_state_home().join("gqlup").join("binaries"))
}
