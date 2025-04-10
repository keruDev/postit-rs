//! Argument parsing utilities with [clap].

use arguments as args;
use clap::{Parser, Subcommand};

/// Contains the arguments struct used.
pub mod arguments {
    use clap::Args;

    use super::subcommands as sub;
    use crate::models::Priority;

    /// Arguments of the 'example' command.
    #[derive(Args, Debug)]
    pub struct Example {
        /// Subcommand the `Example` command will use.
        #[command(subcommand)]
        pub subcommand: sub::Example,
    }

    /// Arguments of the 'flag' command.
    #[derive(Args, Debug)]
    pub struct Flag {
        /// Subcommand the `Flag` command will use.
        #[command(subcommand)]
        pub subcommand: sub::Flag,
    }

    /// Defines a common argument for commands that just use the persister value.
    #[derive(Args, Debug)]
    pub struct Persister {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,
    }

    /// Arguments of the 'add' command.
    #[derive(Args, Debug)]
    pub struct Add {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,

        /// Priority of the task (none, low, med or high).
        #[arg(value_enum)]
        pub priority: Priority,

        /// The content or description of a task.
        pub content: String,
    }

    /// Arguments of the 'check', 'uncheck', and 'drop' commands.
    #[derive(Args, Debug)]
    pub struct Edit {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,

        /// Identifiers of tasks separated by commas.
        #[arg(value_delimiter = ',')]
        pub ids: Vec<u32>,
    }

    /// Arguments of the 'set' command.
    #[derive(Args, Debug)]
    pub struct Set {
        /// Used to read from and save tasks to.
        #[arg(long, short)]
        pub persister: Option<String>,

        /// Subcommand the `Set` command will use.
        #[command(subcommand)]
        pub subcommand: sub::Set,
    }

    /// Arguments of the 'set priority' subcommand.
    #[derive(Args, Debug)]
    pub struct SetPriority {
        /// Priority of the task (none, low, med or high).
        #[arg(value_enum)]
        pub priority: Priority,

        /// Identifiers of tasks separated by commas.
        #[arg(value_delimiter = ',')]
        pub ids: Vec<u32>,
    }

    /// Arguments of the 'set content' subcommand.
    #[derive(Args, Debug)]
    pub struct SetContent {
        /// The content or description of a task.
        pub content: String,

        /// Identifiers of tasks separated by commas.
        #[arg(value_delimiter = ',')]
        pub ids: Vec<u32>,
    }

    /// Arguments of the 'copy' command.
    #[derive(Args, Debug)]
    pub struct Copy {
        /// The persister that contains the tasks.
        pub left: String,

        /// Where the tasks will be copied to.
        pub right: String,
    }

    /// Arguments of the 'config' command.
    #[derive(Args, Debug)]
    pub struct Config {
        /// Subcommand the 'Config' command will use.
        #[command(subcommand)]
        pub subcommand: sub::Config,
    }
}

/// Contains the subcommands available used by parent commands.
pub mod subcommands {
    use clap::Subcommand;

    use super::arguments as args;

    /// Subcommands for setting the task's value.
    #[derive(Subcommand, Debug)]
    pub enum Set {
        /// Changes the 'content' value.
        Content(args::SetContent),
        /// Changes the 'priority' value.
        Priority(args::SetPriority),
    }

    /// Subcommands for managing the config file.
    #[derive(Subcommand, Debug)]
    pub enum Config {
        /// Creates the config file.
        Init,
        /// Shows the config file path.
        Path,
        /// Opens the default editor (via the EDITOR env var) to edit the file
        Edit,
        /// Deletes the config file
        Drop,
    }

    /// Subcommands for the 'Flag' command
    #[derive(Subcommand, Debug)]
    pub enum Flag {
        /// Use example for the 'persister' flag
        Persister,
    }

    /// Subcommands for the 'Example' command
    #[derive(Subcommand, Debug)]
    pub enum Example {
        /// Use example for the 'sample' command
        Sample,
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
        /// Use example for the 'copy' command
        Copy,
        /// Use example for the 'clean' command
        Clean,
        /// Use example for the 'remove' command
        Remove,
        /// Use example for the 'config' command
        Config,
    }
}

/// Contains the different commands available.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Creates a sample of tasks. Useful to test postit's features.
    #[command(alias = "sa")]
    Sample(args::Persister),

    /// Shows a list of the current tasks.
    #[command(alias = "v")]
    View(args::Persister),

    /// Adds a new task to the list.
    #[command(alias = "a")]
    Add(args::Add),

    /// Changes values inside of tasks.
    #[command(alias = "s")]
    Set(args::Set),

    /// Marks a task as checked.
    #[command(alias = "c")]
    Check(args::Edit),

    /// Marks a task as unchecked.
    #[command(alias = "uc")]
    Uncheck(args::Edit),

    /// Deletes a task from the list.
    #[command(alias = "d")]
    Drop(args::Edit),

    /// Creates a copy of a file (can parse formats, like csv to json).
    #[command(alias = "cp")]
    Copy(args::Copy),

    /// Cleans the tasks from a persister
    #[command(alias = "cl")]
    Clean(args::Persister),

    /// Removes a persister completely (file or table)
    #[command(alias = "rm")]
    Remove(args::Persister),

    /// Manages the configuration file (.postit.toml).
    #[command(alias = "conf")]
    Config(args::Config),

    /// Provides use examples for commands
    #[command(alias = "ex")]
    Example(args::Example),

    /// Provides use examples for flags
    #[command(alias = "f")]
    Flag(args::Flag),
}

/// Manages the command and arguments received from console.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, next_line_help = false)]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: Command,
}
