pub mod local;
pub mod remote;

#[derive(usage::Subcommands)]
#[usage(run)]
pub enum Commands {
    Local(local::Args),
    Remote(remote::Args),
}

#[derive(usage::Args)]
#[usage(run)]
pub struct Args {
    #[usage(subcommand)]
    pub command: Commands,
}
