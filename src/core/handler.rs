//! Entry point of the program where all operations to the [Todo] are executed
//! and files are written via a [`SaveFile`] instance.

use crate::fs::file::SaveFile;

use super::args::{Args, Command};
use super::task::Task;
use super::todo::Todo;

/// Handles operations via commands.
pub struct Handler {
    /// Instance of `Todo` with previous tasks loaded from a `SaveFile` instance.
    pub todo: Todo,
}

impl Handler {
    /// Runs the Handler struct based on the args.
    pub fn run(args: Args) {
        match args.command {
            Command::View { path } => Self::view(&path),
            Command::Add { path, task } => Self::add(&path, &task),
            Command::Check { path, ids } => Self::check(&path, &ids),
            Command::Uncheck { path, ids } => Self::uncheck(&path, &ids),
            Command::Drop { path, ids } => Self::drop(&path, &ids),
            Command::Copy { old, new } => Self::copy(&old, &new),
        }
    }

    /// Shows the list of current tasks.
    fn view(path: &str) {
        Todo::read(path).view();
    }

    /// Copies the contents of a file to another.
    fn copy(old: &str, new: &str) {
        let old_path = SaveFile::from(old);
        let new_path = SaveFile::from(new);

        let contents = Todo::from(&old_path);

        new_path.persister.write(&contents);
    }

    /// Adds a new task to the list.
    fn add(path: &str, task: &str) {
        let file = SaveFile::from(path);
        let mut todo = Todo::from(&file);

        todo.add(Task::from(task));
        todo.view();

        file.persister.write(&todo);
    }

    /// Checks the tasks based on the ids passed.
    fn check(path: &str, ids: &[u128]) {
        let file = SaveFile::from(path);
        let mut todo = Todo::from(&file);

        todo.check(ids);
        todo.view();

        file.persister.write(&todo);
    }

    /// Unchecks the tasks based on the ids passed.
    fn uncheck(path: &str, ids: &[u128]) {
        let file = SaveFile::from(path);
        let mut todo = Todo::from(&file);

        todo.uncheck(ids);
        todo.view();

        file.persister.write(&todo);
    }

    /// Drops tasks from the list based on the ids passed.
    fn drop(path: &str, ids: &[u128]) {
        let file = SaveFile::from(path);
        let mut todo = Todo::from(&file);

        todo.drop(ids);
        todo.view();

        file.persister.write(&todo);
    }
}
