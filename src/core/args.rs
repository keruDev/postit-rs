//! Argument parsing utilities with [clap].

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Subcommand, Clone, Debug, ValueEnum)]
/// Options for managing something.
pub enum ConfigOptions {
    /// Creates the config file.
    Init,
    /// Opens the default editor to edit the file.
    Edit,
    /// Deletes the config
    Drop,
}

#[derive(Subcommand, Debug)]
/// Contains the different commands available.
pub enum Command {
    /// Shows a list of the current tasks.
    View {
        /// Used to read from and save tasks to (default: tasks.csv)
        #[arg(long, short, value_name = "PATH")]
        path: Option<String>,
    },
    /// Adds a new task to the list.
    Add {
        /// Used to read from and save tasks to (default: tasks.csv)
        #[arg(long, short, value_name = "PATH")]
        path: Option<String>,

        /// Full task structure (id,content,priority,checked).
        #[arg(value_name = "TASK", help = "Structure: 'id,content,priority,checked'")]
        task: String,
    },
    /// Marks a task as checked so it can be dropped.
    Check {
        /// Used to read from and save tasks to (default: tasks.csv)
        #[arg(long, short, value_name = "PATH")]
        path: Option<String>,

        /// Identifiers of tasks.
        #[arg(value_name = "IDS", help = "Tasks to check")]
        ids: Vec<u128>,
    },
    /// Unchecks a task as if it hasn't been completed.
    Uncheck {
        /// Used to read from and save tasks to (default: tasks.csv)
        #[arg(long, short, value_name = "PATH")]
        path: Option<String>,
        
        /// Identifiers of tasks.
        #[arg(value_name = "IDS", help = "Tasks to uncheck")]
        ids: Vec<u128>,
    },
    /// Deletes a task from the list.
    Drop{
        /// Used to read from and save tasks to (default: tasks.csv)
        #[arg(long, short, value_name = "PATH")]
        path: Option<String>,

        /// Identifiers of tasks.
        #[arg(value_name = "IDS", help = "Tasks to drop")]
        ids: Vec<u128>,
    },
    /// Copies the contents of a file to another.
    Copy {
        /// Used to read from and save tasks to (default: tasks.csv)
        #[arg(value_name = "OLD_PATH", help = "Old path of the tasks file.")]
        old: String,

        /// Used to read from and save tasks to (default: tasks.csv)
        #[arg(value_name = "NEW_PATH", help = "New path of the tasks file.")]
        new: String, 
    },
    /// Manages the configuration file.
    Config {
        #[command(subcommand)]
        /// The option the `Config` command will use.
        option: ConfigOptions,
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, next_line_help = false)]
/// Manages the `Arguments` received by console.
pub struct Arguments {
    /// Command to execute
    #[command(subcommand)]
    pub command: Command,
}
