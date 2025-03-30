//! Argument parsing utilities with [clap].

use clap::Parser;

/// Contains the arguments struct used.
pub mod kind {
    use clap::Args;

    use crate::models::Priority;

    /// Defines common arguments for commands that just use the persister value.
    #[derive(Args, Debug)]
    pub struct PersisterArgs {
        /// Used to read from and save tasks to.
        #[arg(long, short, value_name = "PERSISTER")]
        pub persister: Option<String>,
    }

    /// Defines common arguments for commands related to adding tasks.
    #[derive(Args, Debug)]
    pub struct AddTaskArgs {
        /// Used to read from and save tasks to.
        #[arg(long, short, value_name = "PERSISTER")]
        pub persister: Option<String>,

        /// Priority of the task (none, low, med or high).
        #[arg(value_enum, value_name = "PRIORITY")]
        pub priority: Priority,

        /// The content or description of a task.
        #[arg(value_name = "CONTENT")]
        pub content: String,
    }

    /// Defines common arguments for commands related to editing task values.
    #[derive(Args, Debug)]
    pub struct EditTaskArgs {
        /// Used to read from and save tasks to.
        #[arg(long, short, value_name = "PERSISTER")]
        pub persister: Option<String>,

        /// Identifiers of tasks separated by commas.
        #[arg(value_name = "IDS")]
        pub ids: Vec<u32>,
    }

    /// Defines common arguments for commands related to copying files.
    #[derive(Args, Debug)]
    pub struct CopyTaskArgs {
        /// The persister that contains the tasks.
        #[arg(value_name = "LEFT")]
        pub left: String,

        /// Where the tasks will be copied to.
        #[arg(value_name = "RIGHT")]
        pub right: String,
    }
}

/// Contains the command enums used.
pub mod cmnd {
    use clap::{Subcommand, ValueEnum};

    use super::kind::{AddTaskArgs, CopyTaskArgs, EditTaskArgs, PersisterArgs};

    /// Contains the different commands available.
    #[derive(Subcommand, Debug)]
    pub enum Command {
        /// Shows a list of the current tasks.
        #[command(alias = "v")]
        View(PersisterArgs),

        /// Adds a new task to the list.
        #[command(alias = "a")]
        Add(AddTaskArgs),

        /// Marks a task as checked.
        #[command(alias = "c")]
        Check(EditTaskArgs),

        /// Marks a task as unchecked.
        #[command(alias = "uc")]
        Uncheck(EditTaskArgs),

        /// Deletes a task from the list.
        #[command(alias = "d")]
        Drop(EditTaskArgs),

        /// Creates a copy of a file (can parse formats, like csv to json).
        #[command(alias = "cp")]
        Copy(CopyTaskArgs),

        /// Creates a sample of tasks. Useful to test the program's features.
        #[command(alias = "s")]
        Sample(PersisterArgs),

        /// Cleans the tasks from a persister
        #[command(alias = "cl")]
        Clean(PersisterArgs),

        /// Removes a persister completely (file or table)
        #[command(alias = "rm")]
        Remove(PersisterArgs),

        /// Manages the configuration file (.postit.toml).
        #[command(alias = "conf")]
        Config {
            #[command(subcommand)]
            /// The option the `Config` command will use.
            option: ConfigCommand,
        },
    }

    /// Options for managing the config file.
    #[derive(Subcommand, Clone, Copy, Debug, ValueEnum)]
    pub enum ConfigCommand {
        /// Creates the config file.
        Init,
        /// Opens the default editor to edit the file.
        Edit,
        /// Deletes the config
        Drop,
    }
}

/// Manages the `Arguments` received from console.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, next_line_help = false)]
pub struct Arguments {
    /// Command to execute
    #[command(subcommand)]
    pub command: cmnd::Command,
}
