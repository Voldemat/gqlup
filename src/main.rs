mod cli;

fn main() {
    usage::Run::run(cli::App::parse());
}
