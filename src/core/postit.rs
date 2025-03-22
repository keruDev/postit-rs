//! Contains the `Postit` struct, which is used as a handler that manages the
//! commands received in the passed arguments.
//!
//! For more info about the available commands, check [`Command`][`crate::args::Command`].

use super::args::cmnd::{Command, ConfigCommand};
use super::args::kind::{AddTask, CopyTask, EditTask, UsePersister};
use super::args::Arguments;
use super::{Action, Config};
use crate::models::{Priority, Task, Todo};
use crate::persisters::File;

/// Entry point where all operations are executed.
///
/// Handles operations via commands.
///
/// The [`Todo`] instance is loaded using the desired [`FilePersister`][`crate::persisters::traits::FilePersister`]
/// instance, which is modified when the `Postit` finishes working.
pub struct Postit;

impl Postit {
    /// Runs `Postit` commands based on the args provided.
    pub fn run(args: Arguments) {
        match args.command {
            Command::View(args) => Self::view(args),
            Command::Add(args) => Self::add(args),
            Command::Check(args) => Self::edit(args, Action::Check),
            Command::Uncheck(args) => Self::edit(args, Action::Uncheck),
            Command::Drop(args) => Self::edit(args, Action::Drop),
            Command::Copy(args) => Self::copy(args),
            Command::Clean(args) => Self::clean(args),
            Command::Config { option } => Self::config(option),
        }
    }

    /// Shows the list of current tasks.
    fn view(args: UsePersister) {
        let persister = Config::resolve_persister(args.persister);
        Todo::from(&*persister).view();
    }

    /// Adds a new task to the list.
    fn add(args: AddTask) {
        let persister = Config::resolve_persister(args.persister);
        let mut todo = Todo::from(&*persister);

        let id = todo.tasks.last().map_or(1, |last| last.id + 1);

        let parts: Vec<&str> = args.task.split(',').map(str::trim).collect();

        let content = match parts[0].parse::<u32>() {
            Ok(_n) => panic!("Task element can't be a number"),
            Err(_e) => parts[0],
        };

        let priority =
            if parts.len() > 1 { Priority::from(parts.get(1).unwrap()) } else { Priority::Med };

        let line = format!("{},{},{},{}", id, content, priority, false);
        let task = Task::from(&line);

        todo.add(task);
        todo.view();

        persister.save(&todo);
    }

    /// Edits tasks based on the action passed.
    fn edit(args: EditTask, action: Action) {
        let persister = Config::resolve_persister(args.persister);
        let mut todo = Todo::from(&*persister);

        let changed_ids = match action {
            Action::Check => todo.check(&args.ids),
            Action::Uncheck => todo.uncheck(&args.ids),
            Action::Drop => todo.drop(&args.ids),
        };

        todo.view();

        persister.edit(&changed_ids, action);
    }

    /// Copies the contents of a file to another.
    fn copy(args: CopyTask) {
        let old = args.old;
        let new = args.new;

        File::copy(&old, &new);
    }

    /// Cleans the tasks from a file.
    fn clean(args: UsePersister) {
        let persister = Config::resolve_persister(args.persister);

        persister.clean();
    }

    /// Manages the configuration file.   
    fn config(option: ConfigCommand) {
        Config::manage(&option);
    }
}
