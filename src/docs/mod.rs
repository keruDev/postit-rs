//! Contains commands used for documentation purposes.
//!
//! Their information can be accessed by using the following commands:
//! - postit example <COMMAND>
//! - postit flag <COMMAND>

mod example;
mod flag;

pub use example::Example;
pub use flag::Flag;
