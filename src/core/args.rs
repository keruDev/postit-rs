//! Argument parsing utilities with [clap].

use clap::Parser;

/// Contains the arguments struct used.
pub mod kind {
    use clap::Args;

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

        /// Task structure needed to create a task.
        #[arg(value_name = "TASK", help = "Structure: 'content,priority'")]
        pub task: String,
    }

    /// Defines common arguments for commands related to editing task values.
    #[derive(Args, Debug)]
    pub struct EditTaskArgs {
        /// Used to read from and save tasks to.
        #[arg(long, short, value_name = "PERSISTER")]
        pub persister: Option<String>,

        /// Identifiers of tasks separated by spaces.
        #[arg(value_name = "IDS", help = "Task IDs")]
        pub ids: Vec<u32>,
    }

    /// Defines common arguments for commands related to copying files.
    #[derive(Args, Debug)]
    pub struct CopyTaskArgs {
        /// Where the file is.
        #[arg(value_name = "OLD_PATH", help = "Old path of the tasks file.")]
        pub old: String,

        /// Where the contents will be copied to.
        #[arg(value_name = "NEW_PATH", help = "New path of the tasks file.")]
        pub new: String,
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
        View(PersisterArgs),

        /// Adds a new task to the list.
        Add(AddTaskArgs),

        /// Marks a task as checked.
        Check(EditTaskArgs),

        /// Marks a task as unchecked.
        Uncheck(EditTaskArgs),

        /// Deletes a task from the list.
        Drop(EditTaskArgs),

        /// Creates a copy of a file (can parse formats, like csv to json).
        Copy(CopyTaskArgs),

        /// Creates a sample of tasks. Useful to test the program's features.
        Sample(PersisterArgs),

        /// Cleans the tasks from a persister
        Clean(PersisterArgs),

        /// Removes a persister completely (file or table)
        Remove(PersisterArgs),

        /// Manages the configuration file (.postit.toml).
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
