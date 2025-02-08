use clap::Parser as _;
use postit::args::Arguments;
use postit::Postit;

fn main() {
    Postit::run(Arguments::parse());
}
