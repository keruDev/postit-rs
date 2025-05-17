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
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::missing_docs_in_private_items,
    clippy::dbg_macro,
    clippy::single_match_else,
    clippy::implicit_clone,
    clippy::str_to_string,
    clippy::missing_inline_in_public_items,
    clippy::unused_trait_names,
    clippy::wildcard_enum_match_arm,
    clippy::single_call_fn,
    clippy::as_conversions,
    clippy::panic,
    missing_docs
)]
#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::manual_assert,
    clippy::multiple_crate_versions,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools,
    // TMP
    clippy::expect_used,
)]

pub mod config;
mod core;
pub mod docs;
pub mod models;
mod persisters;

pub use core::*;

pub use persisters::*;
