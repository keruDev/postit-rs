use postit::core::args::Args;
use postit::core::handler::Handler;

use clap::Parser as _;

fn main() {
    Handler::run(Args::parse());
}
