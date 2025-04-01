//! Contains the `Postit` struct, which is used as a handler that manages the
//! commands received in the passed arguments.
//!
//! For more info about the available commands, check [`Command`][`crate::args::Command`].

use super::cli::args::{
    AddTaskArgs, CopyTaskArgs, EditTaskArgs, PersisterArgs, SetContentArgs, SetPriorityArgs,
};
use super::cli::cmnd::{Command, ConfigSubcommand, SetSubcommand};
use super::cli::Cli;
use super::{Action, Config};
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
            Command::View(args) => Self::view(args),
            Command::Add(args) => Self::add(args),
            Command::Set { subcommand } => Self::set(subcommand),
            Command::Check(args) => Self::edit(args, Action::Check),
            Command::Uncheck(args) => Self::edit(args, Action::Uncheck),
            Command::Drop(args) => Self::edit(args, Action::Drop),
            Command::Copy(args) => Self::copy(args),
            Command::Sample(args) => Self::sample(args),
            Command::Clean(args) => Self::clean(args),
            Command::Remove(args) => Self::remove(args),
            Command::Config { subcommand } => Self::config(subcommand),
        }
    }

    /// Shows the list of current tasks.
    fn view(args: PersisterArgs) {
        let persister = Config::resolve_persister(args.persister);
        Todo::from(&*persister).view();
    }

    /// Adds a new task to the list.
    fn add(args: AddTaskArgs) {
        let persister = Config::resolve_persister(args.persister);
        let mut todo = Todo::from(&*persister);

        let id = todo.tasks.last().map_or(1, |last| last.id + 1);

        let line = format!("{},{},{},{}", id, args.content, args.priority, false);
        let task = Task::from(&line);

        todo.add(task);
        persister.save(&todo);

        todo.view();
    }

    /// Changes the values of a task depending on the `SetSubcommand` variant.
    fn set(subcommand: SetSubcommand) {
        match subcommand {
            SetSubcommand::Priority(args) => Self::set_priority(args),
            SetSubcommand::Content(args) => Self::set_content(args),
        }
    }

    /// Changes the `priority` property of tasks (selected by using `args.ids`).
    fn set_priority(args: SetPriorityArgs) {
        let persister = Config::resolve_persister(args.persister);
        let mut todo = Todo::from(&*persister);

        let tasks = todo.get(&args.ids);

        for task in tasks {
            task.priority = args.priority.clone();
        }
    }

    /// Changes the `content` property of tasks (selected by using `args.ids`).
    fn set_content(args: SetContentArgs) {
        let persister = Config::resolve_persister(args.persister);
        let mut todo = Todo::from(&*persister);

        let tasks = todo.get(&args.ids);

        for task in tasks {
            task.content.clone_from(&args.content);
        }
    }

    /// Edits tasks based on the action passed.
    fn edit(args: EditTaskArgs, action: Action) {
        let persister = Config::resolve_persister(args.persister);
        let mut todo = Todo::from(&*persister);

        let changed_ids = match action {
            Action::Check => todo.check(&args.ids),
            Action::Uncheck => todo.uncheck(&args.ids),
            Action::Drop => todo.drop(&args.ids),
        };

        persister.edit(&changed_ids, action);

        todo.view();
    }

    /// Copies the contents of a persister to another.
    ///
    /// # Panics
    /// - If both persisters are the same.
    /// - If the left persister has no tasks.
    /// - If the right persister has tasks.    
    fn copy(args: CopyTaskArgs) {
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
    fn sample(args: PersisterArgs) {
        let persister = Config::resolve_persister(args.persister);
        let todo = Todo::sample();

        persister.save(&todo);

        todo.view();
    }

    /// Cleans the tasks from a file.
    fn clean(args: PersisterArgs) {
        let persister = Config::resolve_persister(args.persister);

        persister.clean();
    }

    /// Removes a persister completely (file or table).
    fn remove(args: PersisterArgs) {
        let persister = Config::resolve_persister(args.persister);

        persister.remove();
    }

    /// Manages the configuration file.   
    fn config(subcommand: ConfigSubcommand) {
        Config::manage(&subcommand);
    }
}
