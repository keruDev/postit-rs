#![warn(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]

#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use
)]

use postit::core::args::Args;
use postit::core::handler::Handler;

use clap::Parser as _;

fn main() {
    Handler::run(Args::parse());
}
