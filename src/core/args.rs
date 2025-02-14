//! Argument parsing utilities with [clap].

use clap::{Parser, Subcommand, ValueEnum};

/// Options for managing something.
#[derive(Subcommand, Clone, Copy, Debug, ValueEnum)]
pub enum ConfigOptions {
    /// Creates the config file.
    Init,
    /// Opens the default editor to edit the file.
    Edit,
    /// Deletes the config
    Drop,
}

/// Contains the different commands available.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Shows a list of the current tasks.
    View {
        /// Used to read from and save tasks to.
        #[arg(long, short, value_name = "PATH")]
        path: Option<String>,
    },
    /// Adds a new task to the list.
    Add {
        /// Used to read from and save tasks to.
        #[arg(long, short, value_name = "PATH")]
        path: Option<String>,

        /// Full task structure (id,content,priority,checked).
        #[arg(value_name = "TASK", help = "Structure: 'id,content,priority,checked'")]
        task: String,
    },
    /// Marks a task as checked.
    Check {
        /// Used to read from and save tasks to.
        #[arg(long, short, value_name = "PATH")]
        path: Option<String>,

        /// Identifiers of tasks.
        #[arg(value_name = "IDS", help = "Tasks to check")]
        ids: Vec<u32>,
    },
    /// Marks a task as unchecked.
    Uncheck {
        /// Used to read from and save tasks to.
        #[arg(long, short, value_name = "PATH")]
        path: Option<String>,

        /// Identifiers of tasks.
        #[arg(value_name = "IDS", help = "Tasks to uncheck")]
        ids: Vec<u32>,
    },
    /// Deletes a task from the list.
    Drop {
        /// Used to read from and save tasks to.
        #[arg(long, short, value_name = "PATH")]
        path: Option<String>,

        /// Identifiers of tasks.
        #[arg(value_name = "IDS", help = "Tasks to drop")]
        ids: Vec<u32>,
    },
    /// Creates a copy of a file (supports other formats, e.g.: csv -> json).
    Copy {
        /// Where the file is.
        #[arg(value_name = "OLD_PATH", help = "Old path of the tasks file.")]
        old: String,

        /// Where the contents will be copied to.
        #[arg(value_name = "NEW_PATH", help = "New path of the tasks file.")]
        new: String,
    },
    /// Manages the configuration file.
    Config {
        #[command(subcommand)]
        /// The option the `Config` command will use.
        option: ConfigOptions,
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
