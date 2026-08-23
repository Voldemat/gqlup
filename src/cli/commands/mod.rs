mod exec;
mod install;
mod list;
mod uninstall;

#[derive(usage::Subcommands)]
#[usage(run, external=exec::exec)]
pub enum Commands {
    List(list::Args),
    Install(install::Args),
    Uninstall(uninstall::Args),
    #[usage(external_subcommand)]
    Exec(Vec<std::ffi::OsString>),
}
