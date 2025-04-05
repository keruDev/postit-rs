use crate::models::{Task, Todo};

pub struct Example;

impl Example {
    fn todo() -> Todo {
        Todo::new(vec![
            Task::from("1,Task,low,false"),
            Task::from("2,Task,med,false"),
            Task::from("3,Task,high,true"),
            Task::from("4,Task,none,true"),
        ])
    }

    #[rustfmt::skip]
    pub fn view() {
        println!("
Usage: postit view
Alias: postit v

Description:
    Takes the 'persister' defined at '.postit.toml' (or the '-p' flag, if provided)
    to show the list of current tasks:

How to use:
    postit view
"
        );

        Self::todo().view();
    }

    #[rustfmt::skip]
    pub fn add() {
        let line = "5,New task,low,false";
        let task = Task::from(line);

        println!("
Usage: postit add <PRIORITY> <CONTENT>
Alias: postit a <PRIORITY> <CONTENT>

Description:
    Creates a task with the format 'id,content,priority,checked': 
    - id: a unique unsigned integer.
    - content: description of the task.
    - priority: high, med, low or none.
    - checked: true or false.

How to use:
    To add a task, just provide the priority and the content of the task:

    postit add low \"New task\"

    The new task will be displayed like this: {task}
"
        );

        let mut todo = Self::todo();

        println!("Before:");
        todo.view();

        println!();
        println!("After:");
        todo.add(task);
        todo.view();
    }

    pub fn set() {
        println!("
Usage: `postit set <COMMAND>`
Alias: `postit s <COMMAND>`

Description:
    Changes the value of task's properties.
    
    These are the available subcommands:
    - content: postit set content <CONTENT> [IDS]...
    - priority: postit set priority <PRIORITY> [IDS]...

How to use:
    To change the content of a task:

    postit set content \"New content\" 2
    
    
    To change the priority of a task:

    postit set priority low 3
"
        );
        
        
        // ```csv
        // 1,Task,low,false
        // 2,New content,med,true  (changed)
        // 3,Task,high,true
        // 4,Task,none,true
        // ```
        
        
        // ```csv
        // 1,Task,low,false
        // 2,Task,med,true
        // 3,Task,low,true         (changed)
        // 4,Task,none,true
        // ```
        
        // let mut todo = Self::todo();

        // println!("Before:");
        // todo.view();

        // println!();
        // println!("After:");
        // todo.set(task);
        // todo.view();
    }

    pub fn check() {}

    pub fn uncheck() {}

    pub fn drop() {}

    pub fn copy() {}

    pub fn sample() {}

    pub fn clean() {}

    pub fn remove() {}

    pub fn config() {}
}
