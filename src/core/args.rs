//! Argument parsing utilities with [clap].

use clap::{Args, Parser, Subcommand, ValueEnum};

/// Options for managing something.
#[derive(Subcommand, Clone, Copy, Debug, ValueEnum)]
pub enum ConfigCommand {
    /// Creates the config file.
    Init,
    /// Opens the default editor to edit the file.
    Edit,
    /// Deletes the config
    Drop,
}

#[derive(Args, Debug)]
/// Defines common arguments for some commands.
pub struct EditTaskArgs {
    /// Used to read from and save tasks to.
    #[arg(long, short, value_name = "PERSISTER")]
    pub persister: Option<String>,

    /// Identifiers of tasks separated by spaces.
    #[arg(value_name = "IDS", help = "Task IDs")]
    pub ids: Vec<u32>,
}

/// Contains the different commands available.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Shows a list of the current tasks.
    View {
        /// Used to read from and save tasks to.
        #[arg(long, short, value_name = "PERSISTER")]
        persister: Option<String>,
    },
    /// Adds a new task to the list.
    Add {
        /// Used to read from and save tasks to.
        #[arg(long, short, value_name = "PERSISTER")]
        persister: Option<String>,

        /// Task structure needed to create a task.
        #[arg(value_name = "TASK", help = "Structure: 'content,priority'")]
        task: String,
    },
    /// Marks a task as checked.
    Check(EditTaskArgs),

    /// Marks a task as unchecked.
    Uncheck(EditTaskArgs),

    /// Deletes a task from the list.
    Drop(EditTaskArgs),

    /// Creates a copy of a file (supports other formats, e.g.: csv -> json).
    Copy {
        /// Where the file is.
        #[arg(value_name = "OLD_PATH", help = "Old path of the tasks file.")]
        old: String,

        /// Where the contents will be copied to.
        #[arg(value_name = "NEW_PATH", help = "New path of the tasks file.")]
        new: String,
    },
    /// Manages the configuration file (.postit.toml or postit.toml).
    Config {
        #[command(subcommand)]
        /// The option the `Config` command will use.
        option: ConfigCommand,
    },
}

/// Manages the `Arguments` received by console.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, next_line_help = false)]
pub struct Arguments {
    /// Command to execute
    #[command(subcommand)]
    pub command: Command,
}
