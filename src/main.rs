mod cli;

fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls ring crypto provider");
    usage::Run::run(cli::App::parse());
}
