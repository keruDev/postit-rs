//! Entry point of the program where all operations to the [Todo] are executed
//! and files are written via a [SaveFile] instance.

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
        let Args { command, ids, task, path } = args.check();

        let file = SaveFile::from(&path);
        let todo = Todo::from(&file);

        let mut handler = Self { todo };

        match command {
            Command::View => return handler.view(),
            Command::Add => handler.add(&task),
            Command::Check => handler.check(&ids),
            Command::Uncheck => handler.uncheck(&ids),
            Command::Drop => handler.drop(&ids),
        };

        file.persister.write(&handler.todo);
    }

    /// Shows the list of current tasks.
    fn view(&mut self) {
        self.todo.view();
    }

    /// Adds a new task to the list.
    fn add(&mut self, task: &str) {
        self.todo.add(Task::from(task));
        self.todo.view();
    }

    /// Checks the tasks based on the ids passed.
    fn check(&mut self, ids: &Vec<u128>) {
        self.todo.check(ids);
        self.todo.view();
    }

    /// Unchecks the tasks based on the ids passed.
    fn uncheck(&mut self, ids: &Vec<u128>) {
        self.todo.uncheck(ids);
        self.todo.view();
    }

    /// Drops tasks from the list based on the ids passed.
    fn drop(&mut self, ids: &Vec<u128>) {
        self.todo.drop(ids);
        self.todo.view();
    }
}
