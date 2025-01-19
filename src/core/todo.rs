//! Collection of existing tasks. This is where major task management is made.

use crate::fs::file::SaveFile;

use super::task::Task;

/// Contains all the Tasks.
#[derive(Clone, Debug, PartialEq)]
pub struct Todo {
    /// List of Tasks.
    pub tasks: Vec<Task>,
}

impl Todo {
    /// Creates a `Todo` instance from a file's contents.
    pub fn from(file: &SaveFile) -> Self {
        Self { tasks: file.persister.tasks() }
    }

    /// Returns tasks based on the ids passed.
    pub fn get(&mut self, ids: &Vec<u128>) -> Vec<&mut Task> {
        self.tasks
            .iter_mut()
            .filter(|task| ids.contains(&task.id))
            .collect()
    }

    /// Shows the current list of tasks.
    pub fn view(&mut self) {
        self.tasks
            .iter_mut()
            .for_each(|task| println!("{}", task.stylize()));
    }

    /// Adds a task to the task list.
    pub fn add(&mut self, mut task: Task) {
        let ids: Vec<u128> = self.tasks.iter().map(|task| task.id).collect();

        if ids.contains(&task.id) {
            if let (Some(&start), Some(&end)) = (ids.first(), ids.iter().max()) {
                let new_id = (start..=end)
                    .find(|n| !ids.contains(n))
                    .unwrap_or(end + 1);

                eprintln!(
                    "ID {} is already used; using {} as an ID",
                    &task.id, new_id
                );

                task.id = new_id;
            }
        }

        self.tasks.push(task);
    }

    /// Marks a task as checked.
    pub fn check(&mut self, ids: &Vec<u128>) {
        for task in self.get(ids) {
            if let Err(e) = task.check() {
                eprintln!("{e}");
            }
        }
    }

    /// Marks a task as unchecked.
    pub fn uncheck(&mut self, ids: &Vec<u128>) {
        for task in self.get(ids) {
            if let Err(e) = task.uncheck() {
                eprintln!("{e}");
            }
        }
    }

    /// Drops a task from the list.
    pub fn drop(&mut self, ids: &Vec<u128>) {
        self.tasks.retain(|task| {
            if ids.contains(&task.id) && !task.checked {
                eprintln!("Task {} can't be dropped; must be checked first", &task.id);
            }

            !(ids.contains(&task.id) && task.checked)
        });
    }
}
