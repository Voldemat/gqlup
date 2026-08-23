#[derive(usage::Args)]
pub struct Args {}

fn run(_: Args) {
    let gql_binaries_path = crate::cli::paths::get_gqlup_binary_install_path();
    for version in
        crate::cli::shared::resolve_local_versions(&gql_binaries_path)
    {
        println!("{version}")
    }
}

impl usage::Run for Args {
    type Output = ();

    fn run(self) -> Self::Output {
        run(self);
        ()
    }
}
