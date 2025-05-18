//! Collection of existing tasks. This is where major task management is made.

use super::Priority;
use crate::cli::subcommands as sub;
use crate::config::Config;
use crate::models::task::Task;
use crate::traits::Persister;

/// Contains all the Tasks.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Todo {
    /// List of Tasks.
    pub tasks: Vec<Task>,
}

impl From<Task> for Vec<Task> {
    #[inline]
    fn from(task: Task) -> Self {
        vec![task]
    }
}

impl From<&Task> for Vec<Task> {
    #[inline]
    fn from(task: &Task) -> Self {
        vec![task.clone()]
    }
}

impl Todo {
    /// Creates a `Todo` instance from a vector of tasks.
    #[inline]
    pub fn new<T: Into<Vec<Task>>>(tasks: T) -> Self {
        Self { tasks: tasks.into() }
    }

    /// Creates a `Todo` instance from a persister's contents.
    #[inline]
    pub fn from(persister: &dyn Persister) -> crate::Result<Self> {
        Ok(Self { tasks: persister.tasks()? })
    }

    /// Returns tasks based on the ids passed.
    #[inline]
    pub fn get(&self, ids: &[u32]) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| ids.contains(&task.id))
            .collect()
    }

    /// Returns tasks based on the ids passed.
    #[inline]
    pub fn get_mut(&mut self, ids: &[u32]) -> Vec<&mut Task> {
        self.tasks
            .iter_mut()
            .filter(|task| ids.contains(&task.id))
            .collect()
    }

    /// Initializes a `Todo` instance with fake data.
    #[inline]
    pub fn sample() -> Self {
        Self::new(vec![
            Task::from("1,Task,high,false"),
            Task::from("2,Task,med,false"),
            Task::from("3,Task,low,true"),
            Task::from("4,Task,none,true"),
        ])
    }

    /// Shows the current list of tasks.
    #[inline]
    pub fn view(&self) -> crate::Result<()> {
        if self.tasks.is_empty() {
            let err = "There are no tasks to print";
            return Err(crate::Error::wrap(err));
        }

        self.tasks.iter().for_each(|task| println!("{task}"));

        Ok(())
    }

    /// Adds a task to the task list.
    #[inline]
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Changes values of tasks based on the `set` subcommand used.
    #[inline]
    pub fn set(&mut self, cmnd: &sub::Set) -> crate::Result<()> {
        match cmnd {
            sub::Set::Priority(args) => self.set_priority(&args.ids, &args.priority),
            sub::Set::Content(args) => self.set_content(&args.ids, &args.content),
        }
    }

    /// Changes the `priority` property of tasks (selected by using `ids`).
    #[inline]
    pub fn set_priority(&mut self, ids: &[u32], priority: &Priority) -> crate::Result<()> {
        if self.tasks.is_empty() {
            let err = "There are no tasks to edit";
            return Err(crate::Error::wrap(err));
        }

        for task in self.get_mut(ids) {
            task.priority = priority.clone();
        }

        Ok(())
    }

    /// Changes the `content` property of tasks (selected by using `ids`).
    #[inline]
    pub fn set_content(&mut self, ids: &[u32], content: &str) -> crate::Result<()> {
        if self.tasks.is_empty() {
            let err = "There are no tasks to edit";
            return Err(crate::Error::wrap(err));
        }

        for task in self.get_mut(ids) {
            task.content = String::from(content);
        }

        Ok(())
    }

    /// Marks a task as checked.
    /// Returns a `Vec<u32>` containing the IDs of the tasks that changed.
    #[inline]
    pub fn check(&mut self, ids: &[u32]) -> crate::Result<Vec<u32>> {
        if self.tasks.is_empty() {
            let err = "There are no tasks to check";
            return Err(crate::Error::wrap(err));
        }

        let mut changed_ids = vec![];

        for task in self.get_mut(ids) {
            match task.check() {
                Ok(_) => changed_ids.push(task.id),
                Err(e) => eprintln!("{e}"),
            }
        }

        Ok(changed_ids)
    }

    /// Marks a task as unchecked.
    /// Returns a `Vec<u32>` containing the IDs of the tasks that changed.
    #[inline]
    pub fn uncheck(&mut self, ids: &[u32]) -> crate::Result<Vec<u32>> {
        if self.tasks.is_empty() {
            let err = "There are no tasks to uncheck";
            return Err(crate::Error::wrap(err));
        }

        let mut changed_ids = vec![];

        for task in self.get_mut(ids) {
            match task.uncheck() {
                Ok(_) => changed_ids.push(task.id),
                Err(e) => eprintln!("{e}"),
            }
        }

        Ok(changed_ids)
    }

    /// Drops a task from the list.
    /// Returns a `Vec<u32>` containing the IDs of the tasks that changed.
    #[inline]
    pub fn drop(&mut self, ids: &[u32]) -> crate::Result<Vec<u32>> {
        if self.tasks.is_empty() {
            let err = "There are no tasks to drop";
            return Err(crate::Error::wrap(err));
        }

        let force_drop = Config::load().unwrap().force_drop;
        let mut changed_ids = vec![];

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

        Ok(changed_ids)
    }
}
