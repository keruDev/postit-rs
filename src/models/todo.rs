//! Collection of existing tasks. This is where major task management is made.

use super::Priority;
use crate::cli::subcommands::Set;
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

    /// Initializes a Todo instance with fake data.
    pub fn sample() -> Self {
        Self::new(vec![
            Task::from("1,Task,low,false"),
            Task::from("2,Task,med,false"),
            Task::from("3,Task,high,true"),
            Task::from("4,Task,none,true"),
        ])
    }

    /// Returns tasks based on the ids passed.
    pub fn get(&mut self, ids: &[u32]) -> Vec<&mut Task> {
        self.tasks
            .iter_mut()
            .filter(|task| ids.contains(&task.id))
            .collect()
    }

    /// Shows the current list of tasks.
    pub fn view(&self) {
        self.tasks.iter().for_each(|task| println!("{task}"));
    }

    /// Adds a task to the task list.
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Changes values of tasks based on the `set` subcommand used.
    pub fn set(&mut self, cmnd: Set) {
        match cmnd {
            Set::Priority(args) => self.set_priority(&args.ids, &args.priority),
            Set::Content(args) => self.set_content(&args.ids, &args.content),
        }
    }

    /// Changes the `priority` property of tasks (selected by using `ids`).
    pub fn set_priority(&mut self, ids: &[u32], priority: &Priority) {
        let tasks = self.get(ids);

        for task in tasks {
            task.priority = priority.clone();
        }
    }

    /// Changes the `content` property of tasks (selected by using `ids`).
    pub fn set_content(&mut self, ids: &[u32], content: &str) {
        let tasks = self.get(ids);

        for task in tasks {
            task.content = String::from(content);
        }
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
