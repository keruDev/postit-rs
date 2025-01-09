#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
)]

#![allow(
    clippy::allow_attributes_without_reason,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::expect_used,
    clippy::implicit_return,
    clippy::min_ident_chars,
    clippy::mod_module_files,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::panic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::return_self_not_must_use,
    clippy::single_call_fn,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
)]

pub mod core;
pub mod fs;

use core::handler::Handler;
use core::args::Args;

use clap::Parser as _;

fn main() {
    Handler::run(Args::parse());
}