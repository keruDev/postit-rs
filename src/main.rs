use postit::Handler;
use postit::args::Arguments;

use clap::Parser as _;

fn main() {
    Handler::run(Arguments::parse());
}