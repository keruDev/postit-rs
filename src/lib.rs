//! # postit-rs - A task manager on your terminal
//!
//!
//! Postit is a CLI utility aimed to help you complete your tasks.
//!
//! It allows you to manage tasks and save a list of them for later use.
//!
//! Some of its features are:
//! - Different task colors depending on priority.
//! - Completed tasks are crossed out.
//! - Support for csv and json files.
//!
//! To get more info, run `postit -h` or take a look to the README file.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::missing_docs_in_private_items,
    missing_docs
)]
#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::or_fun_call,
    clippy::manual_assert,
    clippy::match_wildcard_for_single_variants,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use
)]

mod core;
pub mod docs;
pub mod models;
mod persisters;

pub use core::*;

pub use persisters::*;
