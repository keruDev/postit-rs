//! Contains the `Postit` struct, which is used as a handler that manages the
//! commands received in the passed arguments.
//!
//! For more info about the available commands, check [`Command`][`crate::args::Command`].

use crate::args::{Arguments, Command, ConfigOptions};
use crate::models::{Task, Todo};
use crate::persisters::SaveFile;
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
            Command::Check { persister, ids } => Self::check(persister, &ids),
            Command::Uncheck { persister, ids } => Self::uncheck(persister, &ids),
            Command::Drop { persister, ids } => Self::drop(persister, &ids),
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
        
        todo.add(Task::from(task));
        todo.view();

        persister.save(&todo);
    }

    /// Checks the tasks based on the ids passed.
    fn check(persister: Option<String>, ids: &[u32]) {
        let persister = Config::resolve_persister(persister);
        let mut todo = Todo::from(&*persister);

        todo.check(ids);
        todo.view();

        persister.save(&todo);
    }

    /// Unchecks the tasks based on the ids passed.
    fn uncheck(persister: Option<String>, ids: &[u32]) {
        let persister = Config::resolve_persister(persister);
        let mut todo = Todo::from(&*persister);

        todo.uncheck(ids);
        todo.view();

        persister.save(&todo);
    }

    /// Drops tasks from the list based on the ids passed.
    fn drop(persister: Option<String>, ids: &[u32]) {
        let persister = Config::resolve_persister(persister);
        let mut todo = Todo::from(&*persister);

        todo.drop(ids);
        todo.view();

        persister.save(&todo);
    }

    /// Copies the contents of a file to another.
    fn copy(old: &str, new: &str) {
        SaveFile::copy(old, new);
    }

    /// Manages the configuration file.   
    fn config(option: ConfigOptions) {
        Config::manage(&option);
    }
}
