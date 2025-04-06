pub struct Flag;

impl Flag {
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
        - csv (e.g.: tasks.csv)
        - json (e.g.: tasks.json)
        - xml (e.g.: tasks.xml)

    - Databases
        - SQLite (e.g.: tasks.db, tasks.sqlite or tasks.sqlite3)

How to use:
    postit view --persister tasks.csv
    
    postit view --persister tasks.db

    ..."
        );
    }
}
