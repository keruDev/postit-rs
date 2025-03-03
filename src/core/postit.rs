//! Contains the `Postit` struct, which is used as a handler that manages the
//! commands received in the passed arguments.
//!
//! For more info about the available commands, check [`Command`][`crate::args::Command`].

use crate::args::{Arguments, Command, ConfigOptions};
use crate::models::{Task, Todo};
use crate::persisters::File;
use crate::Config;

use super::args::TaskArgs;

/// Entry point where all operations are executed.
///
/// Handles operations via commands.
///
/// The [`Todo`] instance is loaded using the desired [`FilePersister`][`crate::persisters::traits::FilePersister`]
/// instance, which is modified when the `Postit` finishes working.
pub struct Postit;

impl Postit {
    /// Runs `Postit` commands based on the args provided.
    pub fn run(args: Arguments) {
        match args.command {
            Command::View { persister } => Self::view(persister),
            Command::Add { persister, task } => Self::add(persister, &task),
            Command::Check(args) => Self::check(args),
            Command::Uncheck(args) => Self::uncheck(args),
            Command::Drop(args) => Self::drop(args),
            Command::Copy { old, new } => Self::copy(&old, &new),
            Command::Config { option } => Self::config(option),
        }
    }

    /// Shows the list of current tasks.
    fn view(persister: Option<String>) {
        let persister = Config::resolve_persister(persister);
        Todo::from(&*persister).view();
    }

    /// Adds a new task to the list.
    fn add(persister: Option<String>, task: &str) {
        let persister = Config::resolve_persister(persister);
        let mut todo = Todo::from(&*persister);
        
        todo.add(Task::from(task));
        todo.view();

        persister.save(&todo);
    }

    /// Checks the tasks based on the ids passed.
    fn check(args: TaskArgs) {
        let persister = Config::resolve_persister(args.persister);
        let mut todo = Todo::from(&*persister);

        todo.check(&args.ids);
        todo.view();

        persister.check(&args.ids);
    }

    /// Unchecks the tasks based on the ids passed.
    fn uncheck(args: TaskArgs) {
        let persister = Config::resolve_persister(args.persister);
        let mut todo = Todo::from(&*persister);

        todo.uncheck(&args.ids);
        todo.view();

        persister.uncheck(&args.ids);
    }

    /// Drops tasks from the list based on the ids passed.
    fn drop(args: TaskArgs) {
        let persister = Config::resolve_persister(args.persister);
        let mut todo = Todo::from(&*persister);

        todo.drop(&args.ids);
        todo.view();

        persister.delete(&args.ids);
    }

    /// Copies the contents of a file to another.
    fn copy(old: &str, new: &str) {
        File::copy(old, new);
    }

    /// Manages the configuration file.   
    fn config(option: ConfigOptions) {
        Config::manage(&option);
    }
}
