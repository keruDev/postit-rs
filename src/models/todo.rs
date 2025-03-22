//! Collection of existing tasks. This is where major task management is made.

use crate::models::task::Task;
use crate::traits::Persister;
use crate::Config;

/// Contains all the Tasks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Todo {
    /// List of Tasks.
    pub tasks: Vec<Task>,
}

impl Todo {
    /// Creates a `Todo` instance from a vector of tasks.
    pub const fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }

    /// Creates a `Todo` instance from a file's contents.
    pub fn from(persister: &dyn Persister) -> Self {
        Self { tasks: persister.tasks() }
    }

    /// Adds a task to the task list.
    pub fn one(task: Task) -> Self {
        Self::new(vec![task])
    }

    /// Returns tasks based on the ids passed.
    pub fn get(&mut self, ids: &[u32]) -> Vec<&mut Task> {
        self.tasks
            .iter_mut()
            .filter(|task| ids.contains(&task.id))
            .collect()
    }

    /// Shows the current list of tasks.
    pub fn view(&mut self) {
        self.tasks.iter_mut().for_each(|task| println!("{task}"));
    }

    /// Adds a task to the task list.
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Marks a task as checked.
    pub fn check(&mut self, ids: &[u32]) -> Vec<u32> {
        let mut changed_ids = Vec::<u32>::new();

        for task in self.get(ids) {
            match task.check() {
                Ok(_) => changed_ids.push(task.id),
                Err(e) => eprintln!("{e}"),
            }
        }

        changed_ids
    }

    /// Marks a task as unchecked.
    pub fn uncheck(&mut self, ids: &[u32]) -> Vec<u32> {
        let mut changed_ids = Vec::<u32>::new();

        for task in self.get(ids) {
            match task.uncheck() {
                Ok(_) => changed_ids.push(task.id),
                Err(e) => eprintln!("{e}"),
            }
        }

        changed_ids
    }

    /// Drops a task from the list.
    pub fn drop(&mut self, ids: &[u32]) -> Vec<u32> {
        let force_drop = Config::load().force_drop;
        let mut changed_ids = Vec::<u32>::new();

        self.tasks.retain(|task| {
            let id_exists = ids.contains(&task.id);

            if id_exists {
                if force_drop {
                    changed_ids.push(task.id);
                    return false;
                }

                if !task.checked {
                    eprintln!("Task {} can't be dropped; must be checked first", &task.id);
                    return true;
                }
            }

            let is_retained = id_exists && task.checked;

            if is_retained {
                changed_ids.push(task.id);
            }

            !is_retained
        });

        changed_ids
    }
}
