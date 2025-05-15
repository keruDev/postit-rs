//! Contains the `Postit` struct, which is used as a handler that manages the
//! commands received in the passed arguments.
//!
//! For more info about the available commands, check [`Command`].

#![allow(clippy::single_call_fn)]

use crate::db::Orm;
use crate::fs::File;
use crate::traits::Persister;

use super::cli::{arguments as args, subcommands as sub};
use super::{Action, Cli, Command};
use crate::config::Config;
use crate::docs;
use crate::models::{Task, Todo};

/// Entry point where all operations are executed.
///
/// Handles operations via commands.
///
/// The [`Todo`] instance is loaded using the desired [`FilePersister`][`super::traits::FilePersister`]
/// instance, which is modified when the `Postit` finishes working.
#[non_exhaustive]
pub struct Postit;

impl Postit {
    /// Runs `Postit` commands based on the commands and arguments provided.
    ///
    /// # Errors
    /// If there is any error while operating a persister.
    #[inline]
    pub fn run(cli: Cli) -> super::Result<()> {
        let result = match cli.command {
            Command::Example(args) => {
                Self::example(&args);
                Ok(())
            }
            Command::Flag(args) => {
                Self::flag(&args);
                Ok(())
            }
            Command::Config(args) => Self::config(args),
            Command::View(args) => Self::view(args),
            Command::Add(args) => Self::add(args),
            Command::Set(args) => Self::set(args),
            Command::Check(args) => Self::edit(args, &Action::Check),
            Command::Uncheck(args) => Self::edit(args, &Action::Uncheck),
            Command::Drop(args) => Self::edit(args, &Action::Drop),
            Command::Sample(args) => Self::sample(args),
            Command::Copy(args) => Self::copy(&args),
            Command::Clean(args) => Self::clean(args),
            Command::Remove(args) => Self::remove(args),
        };

        if let Err(e) = &result {
            eprintln!("{e}");
        }

        Ok(())
    }

    /// Builds a persister based on the passed value.
    ///
    /// If the value of `persister` is:
    /// - `Some`: returns itself.
    /// - `None`: returns the persister stored in the config file.
    #[inline]
    pub fn get_persister<T>(persister: Option<T>) -> crate::Result<Box<dyn Persister>>
    where
        T: AsRef<str>,
    {
        let path_or_conn = match persister {
            Some(v) => v.as_ref().to_owned(),
            None => Config::load()?.persister,
        };

        let persister = if path_or_conn.contains("://") || Orm::is_sqlite(&path_or_conn) {
            Orm::from(path_or_conn)?.boxed()
        } else {
            File::from(path_or_conn)?.boxed()
        };

        Ok(persister)
    }

    /// Shows use cases for every other command.
    fn example(args: &args::Example) {
        docs::Command::run(&args.subcommand);
    }

    /// Shows use cases for commonly used flags.
    fn flag(args: &args::Flag) {
        docs::Flag::run(&args.subcommand);
    }

    /// Shows the list of current tasks.
    fn view(args: args::Persister) -> super::Result<()> {
        Self::get_persister(args.persister)?.view()
    }

    /// Adds a new task to the list.
    fn add(args: args::Add) -> super::Result<()> {
        let persister = Self::get_persister(args.persister)?;

        if !persister.exists()? {
            persister.create()?;
        }

        let mut todo = Todo::from(persister.as_ref())?;

        let id = todo.tasks.last().map_or(1, |last| last.id + 1);

        let task = Task::new(id, args.content, args.priority, false);

        todo.add(task);
        persister.save(&todo)?;

        persister.view()
    }

    /// Changes the values of a task depending on the `Set` variant.
    fn set(args: args::Set) -> super::Result<()> {
        let persister = Self::get_persister(args.persister)?;

        if !persister.exists()? {
            let msg = "The persister doesn't exist; add a task first to use this command";
            return Err(super::Error::wrap(msg));
        }

        let mut todo = Todo::from(persister.as_ref())?;

        todo.set(&args.subcommand);

        let (ids, action) = match args.subcommand {
            sub::Set::Content(args) => (args.ids, Action::SetContent),
            sub::Set::Priority(args) => (args.ids, Action::SetPriority),
        };

        persister.edit(&todo, &ids, &action)?;

        persister.view()
    }

    /// Edits tasks based on the action passed.
    fn edit(args: args::Edit, action: &Action) -> super::Result<()> {
        let persister = Self::get_persister(args.persister)?;

        if !persister.exists()? {
            let msg = "The persister doesn't exist; add a task first to use this command";
            return Err(super::Error::wrap(msg));
        }

        let mut todo = Todo::from(persister.as_ref())?;

        let changed_ids = match action {
            Action::Check => todo.check(&args.ids),
            Action::Uncheck => todo.uncheck(&args.ids),
            Action::Drop => todo.drop(&args.ids),
            Action::SetContent | Action::SetPriority => unreachable!(),
        };

        persister.edit(&todo, &changed_ids, action)?;

        persister.view()
    }

    /// Copies the contents of a persister to another.
    ///
    /// # Errors
    /// - If both persisters are the same.
    /// - If the left persister has no tasks.
    /// - If the right persister has tasks.    
    fn copy(args: &args::Copy) -> super::Result<()> {
        if args.left == args.right {
            let msg = "Both persisters are the same";
            return Err(super::Error::wrap(msg));
        }

        let left = Self::get_persister(Some(&args.left))?;

        if left.tasks()? == Vec::new() {
            let msg = format!("The persister '{}' has no tasks to copy", left.to_string());
            return Err(super::Error::wrap(msg));
        }

        let right = Self::get_persister(Some(&args.right))?;

        if !right.exists()? {
            right.create()?;
        }

        let config = Config::load()?;

        if !config.force_copy && right.tasks()? != Vec::new() {
            let msg = format!(
                "The persister '{}' already has tasks.\nSet 'force_copy' to 'true' to overwrite them.",
                right.to_string()
            );

            return Err(super::Error::wrap(msg));
        }

        right.replace(&Todo::from(left.as_ref())?)?;

        if config.drop_after_copy {
            left.remove()?;
        }

        println!("The tasks of '{}' have been copied to '{}'", args.left, args.right);

        right.view()
    }

    /// Populates the persister with fake data for testing purposes.
    fn sample(args: args::Persister) -> super::Result<()> {
        let persister = Self::get_persister(args.persister)?;

        if !persister.exists()? {
            persister.create()?;
        }

        persister.replace(&Todo::sample())?;

        println!("Sample generated at '{}'", persister.to_string());

        persister.view()
    }

    /// Cleans the tasks from a file.
    fn clean(args: args::Persister) -> super::Result<()> {
        Self::get_persister(args.persister)?.clean()
    }

    /// Removes a persister completely (file or table).
    fn remove(args: args::Persister) -> super::Result<()> {
        Self::get_persister(args.persister)?.remove()
    }

    /// Manages the configuration file.   
    fn config(args: args::Config) -> super::Result<()> {
        Config::manage(args.subcommand)?;

        Ok(())
    }
}
