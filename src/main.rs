use postit::Postit;
use postit::args::Arguments;

use clap::Parser as _;

fn main() {
    Postit::run(Arguments::parse());
}