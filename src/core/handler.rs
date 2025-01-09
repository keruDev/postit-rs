use crate::fs::file::SaveFile;

use super::args::{Args, Command};
use super::task::Task;
use super::todo::Todo;


/// Handles operations via commands, load previous tasks,
/// saves the contents of the todo-list for future operations...
pub struct Handler {
    /// Instance of the todo-list with previous tasks.
    pub todo: Todo,
    /// Representation of the file where the todo-list is loaded from and stored in.
    pub file: SaveFile,
}

impl Handler {
    /// Constructor of the Handler struct.
    pub const fn new(todo: Todo, file: SaveFile) -> Self {
        Self { todo, file }
    }

    /// Runs the Handler struct based on the args.
    pub fn run(args: Args) {
        let Args { command, ids, task, path } = args.check();

        let file = SaveFile::from(&path);
        let todo = Todo::new(&file);

        let mut handler = Self::new(todo, file);

        match command {
            Command::Add => handler.add(&task),
            Command::View => handler.view(),
            Command::Check => handler.check(&ids),
            Command::Uncheck => handler.uncheck(&ids),
            Command::Drop => handler.drop(&ids),
        };

        handler.file.write(&handler.todo);
    }

    /// Shows the list of current tasks.
    fn view(&mut self) {
        self.todo.view();
    }

    /// Adds a new task to the list.
    fn add(&mut self, task: &str) {
        self.todo.add(Task::from_str(task));
        self.todo.view();
    }
    
    /// Checks the tasks based on the ids passed.
    fn check(&mut self, ids: &[u128]) {
        self.todo.check(ids);
        self.todo.view();
    }
    
    /// Unchecks the tasks based on the ids passed.
    fn uncheck(&mut self, ids: &[u128]) {
        self.todo.uncheck(ids);
        self.todo.view();
    }
    
    /// Drops tasks from the list based on the ids passed. 
    fn drop(&mut self, ids: &[u128]) {
        self.todo.drop(ids);
        self.todo.view();
    }
}