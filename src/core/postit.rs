//! Contains the `Postit` struct, which is used as a handler that manages the
//! commands received in the passed arguments.
//!
//! For more info about the available commands, check [`Command`][`crate::args::Command`].

use super::args::EditTaskArgs;
use super::{Action, PersisterKind};
use crate::args::{Arguments, Command, ConfigCommand};
use crate::models::{Task, Todo};
use crate::persisters::File;
use crate::Config;

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
            Command::View { persister } => Self::view(persister),
            Command::Add { persister, task } => Self::add(persister, &task),
            Command::Check(args) => Self::edit(args, Action::Check),
            Command::Uncheck(args) => Self::edit(args, Action::Uncheck),
            Command::Drop(args) => Self::edit(args, Action::Drop),
            Command::Copy { old, new } => Self::copy(&old, &new),
            Command::Config { option } => Self::config(option),
        }
    }

    /// Shows the list of current tasks.
    fn view(persister: Option<String>) {
        let persister = Config::resolve_persister(persister);
        Todo::from(&*persister).view();
    }

    /// Adds a new task to the list.
    fn add(persister: Option<String>, task: &str) {
        let persister = Config::resolve_persister(persister);
        let mut todo = Todo::from(&*persister);

        let id = todo.tasks.last().map_or(1, |last| last.id + 1);

        let line = format!("{},{},{}", id, task, false);
        let task = Task::from(&line);

        todo.add(task.clone());
        todo.view();

        // Just saves the new task
        if matches!(persister.kind(), PersisterKind::Db) {
            todo = Todo::new(vec![task]);
        }

        persister.save(&todo);
    }

    /// Edits tasks based on the action passed.
    fn edit(args: EditTaskArgs, action: Action) {
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
    fn copy(old: &str, new: &str) {
        File::copy(old, new);
    }

    /// Manages the configuration file.   
    fn config(option: ConfigCommand) {
        Config::manage(&option);
    }
}
