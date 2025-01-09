use clap::Parser;
use clap::ValueEnum;


#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
/// Contains the different commands available.
pub enum Command {
    /// Shows a list of the current tasks.
    View,
    /// Adds a new task to the list.
    Add,
    /// Marks a task as checked so it can be dropped.
    Check,
    /// Unchecks a task as if it hasn't been completed.
    Uncheck,
    /// Deletes a task from the list.
    Drop,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, next_line_help = false)]
/// Manages the Arguments received by console.
pub struct Args {
    /// Command to execute
    #[arg(long, short, value_name = "COMMAND")]
    #[clap(value_enum, default_value_t = Command::View)]
    pub command: Command,
    
    /// Identifiers for tasks. Used to 'check', 'uncheck' or 'drop'
    #[arg(long, short, value_name = "IDS", value_delimiter = ',', default_value = "0", hide_default_value = true)]
    pub ids: Vec<u128>,

    /// Full task structure (id,content,priority,checked). Used to 'add'
    #[arg(long, short, value_name = "TASK", default_value = "", hide_default_value = true)]
    pub task: String,

    /// Used to read from and save tasks to (default: tasks.csv)
    #[arg(long, short, value_name = "PATH", default_value = "tasks.csv", hide_default_value = true)]
    pub path: String,
}

impl Args {
    /// Checks the arguments received.
    /// 
    /// # Panics
    /// If there is an argument missing for the command passed.
    pub fn check(self) -> Self {
        let Self { command, ids, task, .. } = &self;

        match *command {
            Command::Add => {
                task.is_empty().then(|| panic!("Argument missing: 'task'"));
                self
            },
            Command::Check | Command::Uncheck | Command::Drop => {
                ids.is_empty().then(|| panic!("Argument missing: 'ids'"));
                self
            },
            Command::View => self
        }
    }
}
