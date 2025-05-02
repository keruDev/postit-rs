use clap::Parser as _;
use postit::{Cli, Postit, Result};

fn main() -> Result<()> {
    Postit::run(Cli::parse())
}
