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
            Command::View { path } => Self::view(path),
            Command::Add { path, task } => Self::add(path, &task),
            Command::Check { path, ids } => Self::check(path, &ids),
            Command::Uncheck { path, ids } => Self::uncheck(path, &ids),
            Command::Drop { path, ids } => Self::drop(path, &ids),
            Command::Copy { old, new } => Self::copy(&old, &new),
            Command::Config { option } => Self::config(option),
        }
    }

    /// Shows the list of current tasks.
    fn view(path: Option<String>) {
        Todo::read(&Config::resolve_path(path)).view();
    }

    /// Adds a new task to the list.
    fn add(path: Option<String>, task: &str) {
        let file = SaveFile::from(&Config::resolve_path(path));
        let mut todo = Todo::from(&file);

        todo.add(Task::from(task));
        todo.view();

        file.write(&todo);
    }

    /// Checks the tasks based on the ids passed.
    fn check(path: Option<String>, ids: &[u128]) {
        let file = SaveFile::from(&Config::resolve_path(path));
        let mut todo = Todo::from(&file);

        todo.check(ids);
        todo.view();

        file.write(&todo);
    }

    /// Unchecks the tasks based on the ids passed.
    fn uncheck(path: Option<String>, ids: &[u128]) {
        let file = SaveFile::from(&Config::resolve_path(path));
        let mut todo = Todo::from(&file);

        todo.uncheck(ids);
        todo.view();

        file.write(&todo);
    }

    /// Drops tasks from the list based on the ids passed.
    fn drop(path: Option<String>, ids: &[u128]) {
        let file = SaveFile::from(&Config::resolve_path(path));
        let mut todo = Todo::from(&file);

        todo.drop(ids);
        todo.view();

        file.write(&todo);
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
