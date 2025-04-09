//! Contains the `Postit` struct, which is used as a handler that manages the
//! commands received in the passed arguments.
//!
//! For more info about the available commands, check [`Command`].

use super::cli::arguments as args;
use super::{Action, Cli, Command, Config};
use crate::docs;
use crate::models::{Task, Todo};

/// Entry point where all operations are executed.
///
/// Handles operations via commands.
///
/// The [`Todo`] instance is loaded using the desired [`FilePersister`][`crate::traits::FilePersister`]
/// instance, which is modified when the `Postit` finishes working.
pub struct Postit;

impl Postit {
    /// Runs `Postit` commands based on the commands and arguments provided.
    pub fn run(cli: Cli) {
        match cli.command {
            Command::Example(args) => Self::example(&args),
            Command::Flag(args) => Self::flag(&args),
            Command::Sample(args) => Self::sample(args),
            Command::View(args) => Self::view(args),
            Command::Add(args) => Self::add(args),
            Command::Set(args) => Self::set(args),
            Command::Check(args) => Self::edit(args, Action::Check),
            Command::Uncheck(args) => Self::edit(args, Action::Uncheck),
            Command::Drop(args) => Self::edit(args, Action::Drop),
            Command::Copy(args) => Self::copy(args),
            Command::Clean(args) => Self::clean(args),
            Command::Remove(args) => Self::remove(args),
            Command::Config(args) => Self::config(&args),
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
    fn view(args: args::Persister) {
        let persister = Config::resolve_persister(args.persister);
        Todo::from(&*persister).view();
    }

    /// Adds a new task to the list.
    fn add(args: args::Add) {
        let persister = Config::resolve_persister(args.persister);
        let mut todo = Todo::from(&*persister);

        let id = todo.tasks.last().map_or(1, |last| last.id + 1);

        let line = format!("{},{},{},{}", id, args.content, args.priority, false);
        let task = Task::from(&line);

        todo.add(task);
        persister.save(&todo);

        todo.view();
    }

    /// Changes the values of a task depending on the `Set` variant.
    fn set(args: args::Set) {
        let persister = Config::resolve_persister(args.persister);
        let mut todo = Todo::from(&*persister);

        todo.set(args.subcommand);
        persister.save(&todo);

        todo.view();
    }

    /// Edits tasks based on the action passed.
    fn edit(args: args::Edit, action: Action) {
        let persister = Config::resolve_persister(args.persister);

        persister.edit(&args.ids, action);

        Todo::from(&*persister).view();
    }

    /// Copies the contents of a persister to another.
    ///
    /// # Panics
    /// - If both persisters are the same.
    /// - If the left persister has no tasks.
    /// - If the right persister has tasks.    
    fn copy(args: args::Copy) {
        if args.left == args.right {
            panic!("Both persisters are the same");
        }

        let left = Config::resolve_persister(Some(args.left));

        if left.tasks() == Vec::new() {
            panic!("'{}' has no tasks to copy", left.to_string())
        }

        let right = Config::resolve_persister(Some(args.right));

        let config = Config::load();

        if !config.force_copy && right.tasks() != Vec::new() {
            panic!(
                "'{}' already has tasks. Set 'force_copy' to 'true' to overwrite them.",
                right.to_string()
            );
        }

        right.replace(&Todo::from(&*left));

        if config.drop_after_copy {
            left.remove();
        }

        Todo::new(right.tasks()).view();
    }

    /// Populates the persister with fake data for testing purposes.
    fn sample(args: args::Persister) {
        let persister = Config::resolve_persister(args.persister);
        let todo = Todo::sample();

        persister.save(&todo);

        todo.view();
    }

    /// Cleans the tasks from a file.
    fn clean(args: args::Persister) {
        let persister = Config::resolve_persister(args.persister);

        persister.clean();
    }

    /// Removes a persister completely (file or table).
    fn remove(args: args::Persister) {
        let persister = Config::resolve_persister(args.persister);

        persister.remove();
    }

    /// Manages the configuration file.   
    fn config(args: &args::Config) {
        Config::manage(&args.subcommand);
    }
}
