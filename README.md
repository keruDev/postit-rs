# 📝 postit-rs

[![Build Status](https://github.com/keruDev/postit-rs/workflows/CI/badge.svg)](https://github.com/keruDev/postit-rs/actions)
[![Current Crates.io Version](https://img.shields.io/crates/v/postit.svg)](https://crates.io/crates/postit)
[![Docs.rs](https://img.shields.io/badge/postit-blue.svg?label=docs.rs)](https://docs.rs/postit/latest/postit/)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

Postit is a CLI utility aimed to help you complete your tasks.
You can also save your tasks to keep track of them later.

## From 0.1.x to 0.2.x

The 0.1.x minor marked the beginning of postit's development, but the best is yet to come.
As of 0.1.x, postit featured csv and json file support, as well as some basic commands to
manage tasks and the configuration file.

By bumping the version to 0.2.x, it is intended to mark the first great step of postit to
becoming a more serious product.

To migrate from 0.1.x to 0.2.x, you'll need to change the `--path` flag to `--persister`
(not that hard, I know).

This minor will be focused on providing support for more database systems (MongoDB or MySQL)
along with some more file extensions (XML) and more commands to make task management simpler.

Hope to cross paths in future versions :)

Roadmap:
- [ ] MySQL support
- [ ] MongoDB support
- [ ] XML support
- [ ] Tasks filtering and sorting

## Features

Although `postit` is still in early development, it is alive and keeps growing!
Here are some of its current features and some planned ones as well: 

Customization:
- Configuration file (more info in the [Configuration](#configuration) section below).
- Set your own configuration path using the `POSTIT_CONFIG_PATH` environment variable 
  (by default, `.postit.toml`).

Supported file formats:
- csv
- json

Supported database formats:
- sqlite (.db, .sqlite or .sqlite3)

Display:
- Checked tasks appear crossed out.
- Different colors depending on priority.
  - `high`: red
  - `med`: yellow
  - `low`: blue
  - `none`: white

## Configuration

postit's behavior can be changed using the `.postit.toml` file.

You can check out its possible fields in the [docs](https://docs.rs/postit/latest/postit/struct.Config.html) or down below:
- `persister`: where tasks are stored (the `-p` or `--persister` flag can override this).\
  It can be one of the supported persisters (file or database).
- `force_drop`: if true, allows dropping tasks without them being checked.
- `force_copy`: if true, allows overwriting files on copy if they already exist.
- `drop_after_copy`: if true, drops files after copying.


## Environment variables

- `EDITOR`: used to open your configuration file and edit it.
- `POSTIT_CONFIG_PATH`: specifies where the config file is located (by default, `.postit.toml`).

## Commands

The commands currently available are (click to go to a use example):
- [`view`](#view)
- [`add`](#add)
- [`check`](#check)
- [`uncheck`](#uncheck)
- [`drop`](#drop)
- [`copy`](#copy)
- [`config`](#config)

You can also use the `--help` flag for additional help on every command.

## Examples

Here is a sample of tasks so you try `postit`.

```csv
// tasks.csv

1,Task,low,false
2,Task,med,false
3,Task,high,true
4,Task,none,true
```

### view

Syntax: `postit view`

Takes the `persister` defined at `.postit.toml` (or the `-p` flag, if provided)
to show the list of current tasks:

```csv
postit view

1,Task,low,false
2,Task,med,false
3,Task,high,true
4,Task,none,true
```

### add

Syntax: `postit add <TASK>`

Adds a task with the format `id,content,priority,checked`. 
- **id**: a unique unsigned integer.
- **content**: text contained.
- **priority**: `high`, `med` (default), `low` or `none`.
- **checked**: `true` or `false`.

To add a task, use the format `content,priority`
If priority is left blank, then it will be assigned `med`:

```csv
postit add "New task"

1,Task,low,false
2,Task,med,false
3,Task,high,true
4,Task,none,true
5,New task,med,false    (new element)

```csv
postit add "New task,low"

1,Task,low,false
2,Task,med,false
3,Task,high,true
4,Task,none,true
5,New task,low,false    (new element)
```

### check

Syntax: `postit check <IDS>`

Checks tasks if they are unchecked.

```csv
postit check 2,3

1,Task,low,false
2,Task,med,true         (changed)
3,Task,high,true        (not changed)
4,Task,none,true
```

### uncheck

Syntax: `postit uncheck <IDS>`

Unchecks tasks if they are checked.

```csv
postit uncheck 2,3

1,Task,low,false
2,Task,med,false        (not changed)
3,Task,high,false       (changed)
4,Task,none,true
```

### drop

Syntax: `postit drop <IDS>`

By default, tasks must be checked to be dropped.

```csv
postit drop 2,3

1,Task,low,false
2,Task,med,false        (not dropped)
// 3,Task,high,true     (dropped)
4,Task,none,true
```

You can set the `force_drop` config to `true` to drop tasks wether they are checked or not.

```csv
postit drop 2,3

1,Task,low,false
// 2,Task,med,false     (dropped)
// 3,Task,high,true     (dropped)
4,Task,none,true
```

### copy

Syntax: `postit copy <OLD_PATH> <NEW_PATH>`

Copies a file's contents into another:

```sh
postit copy "tasks.csv" "tasks.json"
```

By default, if the file at `<NEW_PATH>` exists, `postit` will refuse to
overwrite that file in case you are using that file as a backup or you simply
don't want to overwrite it.

You can set the `force_copy` config to `true` to overwrite it anyways.

In the other hand, if you want to copy your file and delete the old one, you can
do it by setting the `drop_after_copy` config to `true`. This will delete the file
located at `<OLD_PATH>`.

### config

Syntax: `postit config <COMMAND>`

Used to manage the config file. These are the available commands:
- `init`: creates the `.postit.toml` file.
- `edit`: executes the editor (`EDITOR` env var) to change configs.
- `drop`: deletes the config file (default values will be used at runtime).

You can also check the [Configuration](#configuration) section where each config
field is explained and there is a link to the official docs.

## Flags

### persister

Syntax: `postit <COMMAND> --persister <PATH_OR_CONN>`

The `--persister` or `-p` flag specifies where the tasks will be read from and saved to.
It can be used on the following commands:
- [view](#view),
- [add](#add),
- [check](#check),
- [uncheck](#uncheck),
- [drop](#drop).

There are currently 3 supported persisters:
```sh
postit <COMMAND> -p tasks.csv
```

```sh
postit <COMMAND> -p tasks.json
```

```sh
postit <COMMAND> -p tasks.db
```

## Testing

To run postit's tests, use this command:
```sh
cargo test -- --test-threads=1
```

You can also use `tarpaulin`, configured in the `.tarpaulin.toml` file.
It is slower, but shows line coverage (not branch coverage):
```sh
cargo tarpaulin -- --test-threads=1
```

The reason why tests are run synchronously is to not overwrite existing files,
control the execution flow (creation and cleanup of temp files) and keep them
as lightweight as possible, as they don't use external dependencies.
