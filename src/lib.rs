#![warn(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic, missing_docs)]

#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use
)]

mod core;
pub mod persisters;

pub use core::Handler;
pub use core::args;
pub use core::models;
