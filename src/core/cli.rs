//! Argument parsing utilities with [clap].

use clap::Parser;

/// Contains the arguments struct used.
pub mod args {
    use clap::Args;

    use crate::models::Priority;

    /// Defines common arguments for commands that just use the persister value.
    #[derive(Args, Debug)]
    pub struct PersisterArgs {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,
    }

    /// Defines common arguments for commands related to adding tasks.
    #[derive(Args, Debug)]
    pub struct AddTaskArgs {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,

        /// Priority of the task (none, low, med or high).
        #[arg(value_enum)]
        pub priority: Priority,

        /// The content or description of a task.
        pub content: String,
    }

    /// Defines common arguments for commands related to editing task values.
    #[derive(Args, Debug)]
    pub struct EditTaskArgs {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,

        /// Identifiers of tasks separated by commas.
        pub ids: Vec<u32>,
    }

    /// Defines common arguments for commands related to editing task values.
    #[derive(Args, Clone, Debug)]
    pub struct SetPriorityArgs {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,

        /// Priority of the task (none, low, med or high).
        #[arg(value_enum)]
        pub priority: Priority,

        /// Identifiers of tasks separated by commas.
        pub ids: Vec<u32>,
    }

    /// Defines common arguments for commands related to editing task values.
    #[derive(Args, Clone, Debug)]
    pub struct SetContentArgs {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,

        /// The content or description of a task.
        pub content: String,

        /// Identifiers of tasks separated by commas.
        pub ids: Vec<u32>,
    }

    /// Defines common arguments for commands related to copying files.
    #[derive(Args, Debug)]
    pub struct CopyTaskArgs {
        /// The persister that contains the tasks.
        pub left: String,

        /// Where the tasks will be copied to.
        pub right: String,
    }
}

/// Contains the command enums used.
pub mod cmnd {
    use clap::Subcommand;

    use super::args::{
        AddTaskArgs, CopyTaskArgs, EditTaskArgs, PersisterArgs, SetContentArgs, SetPriorityArgs,
    };

    /// Contains the different commands available.
    #[derive(Subcommand, Debug)]
    pub enum Command {
        /// Provides use examples for commands
        #[command(alias = "ex")]
        Example {
            /// Subcommand the `Example` command will use.
            #[command(subcommand)]
            subcommand: ExampleSubcommand,
        },

        /// Shows a list of the current tasks.
        #[command(alias = "v")]
        View(PersisterArgs),

        /// Adds a new task to the list.
        #[command(alias = "a")]
        Add(AddTaskArgs),

        /// Changes values inside of tasks.
        #[command(alias = "s")]
        Set {
            /// Subcommand the `Set` command will use.
            #[command(subcommand)]
            subcommand: SetSubcommand,
        },

        /// Marks a task as checked.
        #[command(alias = "c")]
        Check(EditTaskArgs),

        /// Marks a task as unchecked.
        #[command(alias = "uc")]
        Uncheck(EditTaskArgs),

        /// Deletes a task from the list.
        #[command(alias = "d")]
        Drop(EditTaskArgs),

        /// Creates a sample of tasks. Useful to test the program's features.
        #[command(alias = "sa")]
        Sample(PersisterArgs),

        /// Creates a copy of a file (can parse formats, like csv to json).
        #[command(alias = "cp")]
        Copy(CopyTaskArgs),

        /// Cleans the tasks from a persister
        #[command(alias = "cl")]
        Clean(PersisterArgs),

        /// Removes a persister completely (file or table)
        #[command(alias = "rm")]
        Remove(PersisterArgs),

        /// Manages the configuration file (.postit.toml).
        #[command(alias = "conf")]
        Config {
            /// Subcommand the 'Config' command will use.
            #[command(subcommand)]
            subcommand: ConfigSubcommand,
        },
    }

    /// Subcommands for setting the task's value.
    #[derive(Subcommand, Clone, Debug)]
    pub enum SetSubcommand {
        /// Changes the 'content' value.
        Content(SetContentArgs),
        /// Changes the 'priority' value.
        Priority(SetPriorityArgs),
    }

    /// Subcommands for managing the config file.
    #[derive(Subcommand, Clone, Copy, Debug)]
    pub enum ConfigSubcommand {
        /// Creates the config file.
        Init,
        /// Opens the default editor to edit the file.
        Edit,
        /// Deletes the config
        Drop,
    }

    /// Subcommands for the 'Example' command
    #[derive(Subcommand, Clone, Copy, Debug)]
    pub enum ExampleSubcommand {
        /// Use example for the 'view' command
        View,
        /// Use example for the 'add' command
        Add,
        /// Use example for the 'set' command
        Set,
        /// Use example for the 'check' command
        Check,
        /// Use example for the 'uncheck' command
        Uncheck,
        /// Use example for the 'drop' command
        Drop,
        /// Use example for the 'sample' command
        Sample,
        // Copy,
        // Clean,
        // Remove,
        // Config,
    }
}

/// Manages the command and arguments received from console.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, next_line_help = false)]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: cmnd::Command,
}
