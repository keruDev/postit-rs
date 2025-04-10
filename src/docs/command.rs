//! Contains examples of how to use every command, including their usage, alias
//! a description and a use example to showcase the command's functionalities.

use std::fs;

use crate::cli::subcommands as sub;
use crate::models::{Priority, Task, Todo};
use crate::Config;

/// Contains use cases for every command.
pub struct Command;

impl Command {
    /// Uses the [`sub::Example`] value passed to show its corresponding example.
    pub fn run(cmnd: &sub::Example) {
        match cmnd {
            sub::Example::Sample => Self::sample(),
            sub::Example::View => Self::view(),
            sub::Example::Add => Self::add(),
            sub::Example::Set => Self::set(),
            sub::Example::Check => Self::check(),
            sub::Example::Uncheck => Self::uncheck(),
            sub::Example::Drop => Self::drop(),
            sub::Example::Copy => Self::copy(),
            sub::Example::Clean => Self::clean(),
            sub::Example::Remove => Self::remove(),
            sub::Example::Config => Self::config(),
        }
    }

    /// Use case of the 'sample' command.
    pub fn sample() {
        println!(
            "
Usage: postit sample [--persister|-p]
Alias: postit sa ...

Description:
    Populates a persister with fake data so you can test other commands.

How to use:
    postit sample -p tasks.csv

Sample:"
        );

        Todo::sample().view();
    }

    /// Use case of the 'view' command.
    pub fn view() {
        println!(
            "
Usage: postit view [--persister|-p]
Alias: postit v ...

Description:
    Shows the list of tasks stored in a persister.

How to use:
    postit view -p tasks.csv
"
        );

        Todo::sample().view();
    }

    /// Use case of the 'add' command.
    pub fn add() {
        let line = "5,New task,low,false";
        let task = Task::from(line);

        println!(
            "
Usage: postit add <PRIORITY> <CONTENT> [--persister|-p]
Alias: postit a ...

Description:
    Creates a task with the format 'id,content,priority,checked': 
    - id: a unique unsigned integer.
    - content: description of the task.
    - priority: high, med, low or none.
    - checked: true or false.

    To add a task, just provide the priority and the content of the task.

How to use:
    postit add low \"New task\" -p tasks.csv

    The new task will be displayed like this: {task}
"
        );

        let mut todo = Todo::sample();

        println!("Before:");

        todo.view();

        println!();
        println!("After:");

        todo.add(task);
        todo.view();
    }

    /// Use case of the 'set' command.
    pub fn set() {
        fn set_content() {
            let mut todo = Todo::sample();
            let new_content = "New content";
            let line = format!("2,{new_content},med,false");
            let task = Task::from(&line);

            println!(
                "
How to use (content):
    postit set content \"{}\" 2

    Old task: {}
    New task: {}
",
                new_content, todo.tasks[1], task
            );

            println!("Before:");

            todo.view();

            println!();
            println!("After:");

            todo.set_content(&[2], new_content);
            todo.view();
        }

        fn set_priority() {
            let mut todo = Todo::sample();
            let new_priority = Priority::Low;
            let line = format!("2,Task,{new_priority},false");
            let task = Task::from(&line);

            println!(
                "
How to use (priority):
    postit set priority low 3

    Old task: {}
    New task: {}
",
                todo.tasks[1], task
            );

            println!("Before:");

            todo.view();

            println!();
            println!("After:");

            todo.set_priority(&[2], &new_priority);
            todo.view();
        }

        println!(
            "
Usage: postit set <COMMAND> [--persister|-p]
Alias: postit s ...

Description:
    Changes the value of task's properties.
    
    These are the available subcommands:
    - content: postit set content <CONTENT> [IDS]...
    - priority: postit set priority <PRIORITY> [IDS]..."
        );

        set_content();
        set_priority();
    }

    /// Use case of the 'check' command.
    pub fn check() {
        println!(
            "
Usage: postit check <IDS> [--persister|-p]
Alias: postit c ...

Description:
    Checks tasks if they are unchecked.

How to use:
    postit check 2,3 -p tasks.csv
"
        );

        let mut todo = Todo::sample();

        println!("Before:");

        todo.view();

        println!();
        println!("After:");

        todo.check(&[2, 3]);
        todo.view();
    }

    /// Use case of the 'uncheck' command.
    pub fn uncheck() {
        println!(
            "
Usage: postit uncheck <IDS> [--persister|-p]
Alias: postit uc ...

Description:
    Unchecks tasks if they are checked.

How to use:
    postit uncheck 2,3 -p tasks.csv
"
        );

        let mut todo = Todo::sample();

        println!("Before:");

        todo.view();

        println!();
        println!("After:");

        todo.uncheck(&[2, 3]);
        todo.view();
    }

    /// Use case of the 'drop' command.
    pub fn drop() {
        fn force_drop() {
            println!(
                "
Config:
    You can set the 'force_drop' config to 'true' to drop tasks whether 
    they are checked or not.
"
            );

            let path = ".example_postit.toml";
            std::env::set_var("POSTIT_ROOT", path);

            Config { force_drop: true, ..Config::default() }.save();

            let mut todo = Todo::sample();

            println!("Before:");

            todo.view();

            println!();
            println!("After:");

            todo.drop(&[2, 3]);
            todo.view();

            fs::remove_file(path).unwrap();
        }

        println!(
            "
Usage: postit drop <IDS> [--persister|-p]
Alias: postit d ...

Description:
    By default, only checked tasks can be dropped.

How to use:
    postit drop 2,3 -p tasks.csv
"
        );

        let mut todo = Todo::sample();

        println!("Before:");

        todo.view();

        println!();
        println!("After:");

        todo.drop(&[2, 3]);
        todo.view();

        force_drop();
    }

    /// Use case of the 'copy' command.
    pub fn copy() {
        println!(
            "
Usage: postit copy <LEFT> <RIGHT>
Alias: postit cp ...

Description:
    Copies a persister's contents into another, meaning you can use this
    command to 'translate' tasks to a different format.

How to use:
    postit copy tasks.csv tasks.json
    
    postit copy tasks.xml tasks.db

    postit copy tasks.db tasks.json

    ...

Config:
    By default, if the persister at '<RIGHT>' exists, 'postit' will refuse to
    overwrite its tasks in case you are using that persister as a backup or you
    simply don't want to overwrite it.

    You can set the 'force_copy' config to 'true' to overwrite it anyways.

    If you want to copy your tasks and delete the '<LEFT>' persister, you can do so
    by setting the 'drop_after_copy' config to 'true'. This will delete the file or
    table located at '<LEFT>'."
        );
    }

    /// Use case of the 'clean' command.
    pub fn clean() {
        println!(
            "
Usage: postit clean [--persister|-p]
Alias: postit cl ...

Description:
    Deletes all tasks from a persister.

How to use:
    postit clean"
        );
    }

    /// Use case of the 'remove' command.
    pub fn remove() {
        println!(
            "
Usage: postit remove [--persister|-p]
Alias: postit rm ...

Description:
    Deletes the persister completely (file or table).

How to use:
    postit remove"
        );
    }

    /// Use case of the 'config' command.
    pub fn config() {
        println!(
            "
Usage: postit config <COMMAND>
Alias: postit conf ...

Description:
    Manages the config file. Uses the 'POSTIT_ROOT' environment variable
    to locate the file.

    Available subcommands:
    - init: creates the .postit.toml file.
    - edit: executes the editor (EDITOR env var) to change configs.
    - drop: deletes the config file (default values will be used at runtime).

How to use:
    postit config init

    postit config edit

    postit config drop

Default config:
    After running 'postit config init', postit will generate a file with the
    default settings:

    - persister: where tasks are stored (the '-p' or '--persister' flag can override this).
    It can be one of the supported persisters (file or database).

    - force_drop: if true, allows dropping tasks even if they are not checked.

    - force_copy: if true, allows overwriting tasks on populated persisters when
      using the 'copy' command.

    - drop_after_copy: if true, drops a persister (file or table) after copying.
    
    You can also check https://docs.rs/postit/latest/postit/struct.Config.html for more info."
        );
    }
}
