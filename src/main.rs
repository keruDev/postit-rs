use clap::Parser as _;
use postit::{Cli, Postit};

fn main() {
    Postit::run(Cli::parse());
}
