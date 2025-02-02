use crate::persisters::base::SaveFile;

use super::args::{Arguments, Command};
use super::models::{Task, Todo};
use super::Config;

/// Entry point where all operations are executed.
/// 
/// Handles operations via commands.
/// 
/// The [`Todo`] instance is loaded using the desired [`Persister`][`crate::persisters::traits::Persister`]
/// instance, which is modified when the `Postit` finishes working.
pub struct Postit;

impl Postit {
    /// Runs the Postit struct based on the args.
    pub fn run(args: Arguments) {
        match args.command {
            Command::View { path } => Self::view(&Config::resolve_path(path)),
            Command::Add { path, task } => Self::add(&Config::resolve_path(path), &task),
            Command::Check { path, ids } => Self::check(&Config::resolve_path(path), &ids),
            Command::Uncheck { path, ids } => Self::uncheck(&Config::resolve_path(path), &ids),
            Command::Drop { path, ids } => Self::drop(&Config::resolve_path(path), &ids),
            Command::Copy { old, new } => Self::copy(&old, &new),
        }
    }

    /// Shows the list of current tasks.
    fn view(path: &str) {
        Todo::read(path).view();
    }

    /// Copies the contents of a file to another.
    fn copy(old: &str, new: &str) {
        SaveFile::copy(old, new);
    }

    /// Adds a new task to the list.
    fn add(path: &str, task: &str) {
        let file = SaveFile::from(path);
        let mut todo = Todo::from(&file);

        todo.add(Task::from(task));
        todo.view();

        file.write(&todo);
    }

    /// Checks the tasks based on the ids passed.
    fn check(path: &str, ids: &[u128]) {
        let file = SaveFile::from(path);
        let mut todo = Todo::from(&file);

        todo.check(ids);
        todo.view();

        file.write(&todo);
    }

    /// Unchecks the tasks based on the ids passed.
    fn uncheck(path: &str, ids: &[u128]) {
        let file = SaveFile::from(path);
        let mut todo = Todo::from(&file);

        todo.uncheck(ids);
        todo.view();

        file.write(&todo);
    }

    /// Drops tasks from the list based on the ids passed.
    fn drop(path: &str, ids: &[u128]) {
        let file = SaveFile::from(path);
        let mut todo = Todo::from(&file);

        todo.drop(ids);
        todo.view();

        file.write(&todo);
    }
}
