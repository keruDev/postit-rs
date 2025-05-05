//! Contains the `Postit` struct, which is used as a handler that manages the
//! commands received in the passed arguments.
//!
//! For more info about the available commands, check [`Command`].

#![allow(clippy::single_call_fn)]

use super::cli::{arguments as args, subcommands as sub};
use super::{Action, Cli, Command};
use crate::config::Config;
use crate::docs;
use crate::models::{Task, Todo};

/// Entry point where all operations are executed.
///
/// Handles operations via commands.
///
/// The [`Todo`] instance is loaded using the desired [`FilePersister`][`crate::traits::FilePersister`]
/// instance, which is modified when the `Postit` finishes working.
#[non_exhaustive]
pub struct Postit;

impl Postit {
    /// Runs `Postit` commands based on the commands and arguments provided.
    ///
    /// # Errors
    /// If there is any error while operating a persister.
    #[inline]
    pub fn run(cli: Cli) -> crate::Result<()> {
        match cli.command {
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
            Command::Check(args) => Self::edit(args, Action::Check),
            Command::Uncheck(args) => Self::edit(args, Action::Uncheck),
            Command::Drop(args) => Self::edit(args, Action::Drop),
            Command::Sample(args) => Self::sample(args),
            Command::Copy(args) => Self::copy(args),
            Command::Clean(args) => Self::clean(args),
            Command::Remove(args) => Self::remove(args),
        }
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
    fn view(args: args::Persister) -> crate::Result<()> {
        let persister = Config::resolve_persister(args.persister)?;
        let tasks = persister.tasks()?;

        Todo::new(tasks).view();

        Ok(())
    }

    /// Adds a new task to the list.
    fn add(args: args::Add) -> crate::Result<()> {
        let persister = Config::resolve_persister(args.persister)?;
        let mut todo = Todo::from(&*persister)?;

        let id = todo.tasks.last().map_or(1, |last| last.id + 1);

        let task = Task::new(id, args.content, args.priority, false);

        todo.add(task);
        persister.save(&todo)?;

        todo.view();

        Ok(())
    }

    /// Changes the values of a task depending on the `Set` variant.
    fn set(args: args::Set) -> crate::Result<()> {
        let persister = Config::resolve_persister(args.persister)?;
        let mut todo = Todo::from(&*persister)?;

        todo.set(&args.subcommand);

        let (ids, action) = match args.subcommand {
            sub::Set::Content(args) => (args.ids, Action::SetContent),
            sub::Set::Priority(args) => (args.ids, Action::SetPriority),
        };

        persister.edit(&todo, &ids, action)?;

        todo.view();

        Ok(())
    }

    /// Edits tasks based on the action passed.
    fn edit(args: args::Edit, action: Action) -> crate::Result<()> {
        let persister = Config::resolve_persister(args.persister)?;
        let mut todo = Todo::from(&*persister)?;

        let changed_ids = match action {
            Action::Check => todo.check(&args.ids),
            Action::Uncheck => todo.uncheck(&args.ids),
            Action::Drop => todo.drop(&args.ids),
            Action::SetContent | Action::SetPriority => unreachable!(),
        };

        persister.edit(&todo, &changed_ids, action)?;

        Todo::from(&*persister)?.view();

        Ok(())
    }

    /// Copies the contents of a persister to another.
    ///
    /// # Errors
    /// - If both persisters are the same.
    /// - If the left persister has no tasks.
    /// - If the right persister has tasks.    
    fn copy(args: args::Copy) -> crate::Result<()> {
        if args.left == args.right {
            let msg = "Both persisters are the same";
            return Err(crate::Error::Other(msg.into()));
        }

        let left = Config::resolve_persister(Some(args.left))?;

        if left.tasks()? == Vec::new() {
            let msg = format!("'{}' has no tasks to copy", left.to_string());
            return Err(crate::Error::Other(msg.into()));
        }

        let right = Config::resolve_persister(Some(args.right))?;

        let config = Config::load()?;

        if !config.force_copy && right.tasks()? != Vec::new() {
            let msg = format!(
                "'{}' already has tasks. Set 'force_copy' to 'true' to overwrite them.",
                right.to_string()
            );

            return Err(crate::Error::Other(msg.into()));
        }

        right.replace(&Todo::from(&*left)?)?;

        if config.drop_after_copy {
            left.remove()?;
        }

        Todo::new(right.tasks()?).view();

        Ok(())
    }

    /// Populates the persister with fake data for testing purposes.
    fn sample(args: args::Persister) -> crate::Result<()> {
        let persister = Config::resolve_persister(args.persister)?;
        let todo = Todo::sample();

        persister.clean()?;
        persister.save(&todo)?;

        todo.view();

        Ok(())
    }

    /// Cleans the tasks from a file.
    fn clean(args: args::Persister) -> crate::Result<()> {
        let persister = Config::resolve_persister(args.persister)?;

        persister.clean()
    }

    /// Removes a persister completely (file or table).
    fn remove(args: args::Persister) -> crate::Result<()> {
        let persister = Config::resolve_persister(args.persister)?;

        persister.remove()
    }

    /// Manages the configuration file.   
    fn config(args: args::Config) -> crate::Result<()> {
        Config::manage(args.subcommand)?;

        Ok(())
    }
}
