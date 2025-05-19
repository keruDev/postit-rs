//! Contains commands used for documentation purposes.
//!
//! Their information can be accessed by using the following commands:
//! - postit docs <COMMAND>
//! - postit flag <COMMAND>

mod command;
mod flag;

pub use command::Command;
pub use flag::Flag;
