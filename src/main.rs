use clap::Parser as _;
use postit::{Postit, Cli};

fn main() {
    Postit::run(Cli::parse());
}
