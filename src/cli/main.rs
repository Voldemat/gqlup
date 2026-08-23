const GQLUP_VERSION: &str = match option_env!("GQLUP_VERSION") {
    Some(v) => v,
    None => "unspecified",
};

#[derive(usage::Cli)]
#[usage(bin = "gqlup", version = GQLUP_VERSION, run)]
pub struct App {
    #[usage(subcommand)]
    command: super::commands::Commands,
}
