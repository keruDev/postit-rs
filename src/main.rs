use clap::Parser as _;
use postit::{Cli, Postit};

fn main() {
    if let Err(e) = Postit::run(Cli::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
