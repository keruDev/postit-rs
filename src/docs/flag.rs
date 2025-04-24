//! Contains examples of how to use every flag, including their long and short
//! forms, a description and a use example to showcase the flags's functionalities.

use crate::cli::subcommands as sub;

/// Contains use cases for every flag.
pub struct Flag;

impl Flag {
    /// Uses the [`sub::Flag`] value passed to show its corresponding example.
    pub fn run(flag: &sub::Flag) {
        match flag {
            sub::Flag::Persister => Self::persister(),
        }
    }

    /// Use case of the 'persister' flag.
    pub fn persister() {
        println!(
            "
Usage: postit <COMMAND> [--persister | -p] <PATH_OR_CONN>

Description:
    Specifies the persister where the tasks will be read from and saved to.

    It can be a file (CSV, JSON, etc.) or a database (SQLite, etc.). The persister
    is defined in '.postit.toml', or you can override it with the `-p` flag.

    There are currently 4 supported persisters:

    - Files
      - csv             (e.g.: tasks.csv)
      - json            (e.g.: tasks.json)
      - xml             (e.g.: tasks.xml)

    - Databases
      - SQLite          (e.g.: tasks.db, tasks.sqlite or tasks.sqlite3)
      - MongoDB         (e.g.: mongodb://user:pass@host:port)
      - MongoDB Atlas   (e.g.: mongodb+srv://user:pass@cluster)

How to use:
    postit view --persister tasks.csv

    postit view --persister tasks.db

    postit view --persister mongodb://localhost:27017
    
    postit view --persister mongodb+srv://my_user:my_pass@cluster.mongodb.net
    
    ..."
        );
    }
}
