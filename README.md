# 📝 postit-rs

[![Coverage Status](https://coveralls.io/repos/github/keruDev/postit-rs/badge.svg?branch=master)](https://coveralls.io/github/keruDev/postit-rs?branch=master)
[![Build Status](https://github.com/keruDev/postit-rs/workflows/CI/badge.svg)](https://github.com/keruDev/postit-rs/actions)
[![Current Crates.io Version](https://img.shields.io/crates/v/postit.svg)](https://crates.io/crates/postit)
[![Docs.rs](https://img.shields.io/badge/postit-blue.svg?label=docs.rs)](https://docs.rs/postit/latest/postit/)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

Postit is a CLI utility aimed to help you complete your tasks.
You can also save your tasks to keep track of them later.

## Index

Here is an index of this file to make it easier for you to navigate:

- [Getting started](#getting-started): describes installation and first steps.
- [From 0.1.x to 0.2.x](#from-01x-to-02x): brief migration guide.
- [Features](#features): postit's functionalities and new additions roadmap. 
- [Configuration](#configuration): describes configuration options.
- [Environment variables](#environment-variables): environment variables used.
- [Commands](#commands): all commands available, including a description and use example.
- [Flags](#flags): all flags available, including a description and what commands support them.
- [Development](#development): things to take into account if you want to contribute to postit.

## Getting started

To install `postit`, just use:

```sh
cargo install postit
```

After installing, run the `help` command to display a list of all possible commands:

```sh
postit help
```

`postit` uses a configuration file called `.postit.toml`. You can tell `postit`
where this file is by using the `POSTIT_CONFIG_PATH` env var. After setting it,
use the command below to generate the config structure:

```sh
postit config init
```

## From 0.1.x to 0.2.x

The 0.1.x minor marked the beginning of postit's development, but the best is
yet to come. As of 0.1.x, postit featured csv and json file support, as well
as some basic commands to manage tasks and the configuration file.

By bumping the version to 0.2.x, it is intended to mark the first great step
of postit to becoming a more serious product.

To migrate from 0.1.x to 0.2.x, you'll need to change the `--path` flag to 
`--persister` (pretty simple, right?).

This minor will be focused on providing support for more database systems
(MongoDB or MySQL) along with some more file extensions (XML) and more commands
to make task management simpler.

Hope to cross paths in future versions :)

## Features

Although `postit` is still in early development, it is alive and keeps growing!
Here are some of its current features and some planned ones as well: 

Features:
- [Commands](#commands) and [flags](#flags) to manage tasks and files.
- Supported file and database formats to persist data (described in the [persisters](#persister) flag).
- Configuration file to change postit's behavior (more info in the [Configuration](#configuration) section).
- Tasks are displayed differently depending on their priority and whether they are checked or not.

Roadmap:
- [x] XML support
- [ ] MongoDB support
- [ ] MySQL support
- [ ] Tasks filtering and sorting

## Configuration

postit's behavior can be changed using the `.postit.toml` file.

You can check out its possible fields in the [docs](https://docs.rs/postit/latest/postit/struct.Config.html) or down below:
- `persister`: where tasks are stored (the `-p` or `--persister` flag can override this).\
  It can be one of the supported persisters (file or database).
- `force_drop`: if true, allows dropping tasks even if they are not checked.
- `force_copy`: if true, allows overwriting tasks on populated persisters when
   using the [`copy`](#copy) command.
- `drop_after_copy`: if true, drops a persister (file or table) after copying.

## Environment variables

Here is a list of the environment variables currently used:

- `EDITOR`: used to open your configuration file and edit it.
- `POSTIT_CONFIG_PATH`: where the config file is located (by default, `.postit.toml`).

## Commands

The commands currently available are (click to go to a use example):
- [`sample`](#sample)
- [`view`](#view)
- [`add`](#add)
- [`check`](#check)
- [`uncheck`](#uncheck)
- [`drop`](#drop)
- [`copy`](#copy)
- [`clean`](#clean)
- [`remove`](#remove)
- [`config`](#config)

You can also use the `--help` flag for additional help on every command.

### sample

Syntax: `postit sample`

Alias: `postit sa`

Populates a persister with fake data so you can test other commands. This command
takes the `persister` defined at `.postit.toml` (or the `-p` flag, if provided):

```sh
postit sample
```

```csv
1,Task,low,false
2,Task,med,false
3,Task,high,true
4,Task,none,true
```

### check

Syntax: `postit check <IDS>`

Alias: `postit c <IDS>`

Checks tasks if they are unchecked.

```sh
postit check 2,3
```

```csv
1,Task,low,false
2,Task,med,true         (changed)
3,Task,high,true        (not changed)
4,Task,none,true
```

### uncheck

Syntax: `postit uncheck <IDS>`

Alias: `postit uc <IDS>`

Unchecks tasks if they are checked.

```sh
postit uncheck 2,3
```

```csv
1,Task,low,false
2,Task,med,false        (not changed)
3,Task,high,false       (changed)
4,Task,none,true
```

### drop

Syntax: `postit drop <IDS>`

Alias: `postit d <IDS>`

By default, only checked tasks can be dropped.

```sh
postit drop 2,3
```

```csv
1,Task,low,false
2,Task,med,false        (not dropped)
// 3,Task,high,true     (dropped)
4,Task,none,true
```

You can set the `force_drop` config to `true` to drop tasks whether they are checked or not.

```sh
postit drop 2,3
```

```csv
1,Task,low,false
// 2,Task,med,false     (dropped)
// 3,Task,high,true     (dropped)
4,Task,none,true
```

### copy

Syntax: `postit copy <LEFT> <RIGHT>`

Alias: `postit cp <LEFT> <RIGHT>`

Copies a persister's contents into another:

```sh
postit copy tasks.csv tasks.json
```

You can also copy data from a file persister to a database persister:

```sh
postit copy tasks.db tasks.xml
```

By default, if the persister at `<RIGHT>` exists, `postit` will refuse to
overwrite its tasks in case you are using that persister as a backup or you
simply don't want to overwrite it.

You can set the `force_copy` config to `true` to overwrite it anyways.

If you want to copy your tasks and delete the `<LEFT>` persister, you can do so
by setting the `drop_after_copy` config to `true`. This will delete the file or
table located at `<LEFT>`.

### clean

Syntax: `postit clean`

Alias: `postit cl`

Deletes all tasks from a persister. This command takes the `persister` defined
at `.postit.toml` (or the `-p` flag, if provided):

```sh
postit clean
```

### remove

Syntax: `postit remove`

Alias: `postit rm`

Deletes the persister completely (file or table). This command takes the
`persister` defined at `.postit.toml` (or the `-p` flag, if provided):

```sh
postit remove
```

### config

Syntax: `postit config <COMMAND>`

Alias: `postit conf <COMMAND>`

Used to manage the config file. These are the available subcommands:
- `init`: creates the `.postit.toml` file.
- `edit`: executes the editor (`EDITOR` env var) to change configs.
- `drop`: deletes the config file (default values will be used at runtime).

You can also check the [Configuration](#configuration) section where each config
field is explained and there is a link to the official docs.

## Flags

### persister

Syntax: `postit <COMMAND> [--persister | -p] <PATH_OR_CONN>`

The `--persister` or `-p` flag specifies where the tasks will be read from and saved to.

A persister is the storage where tasks are saved. It can be a file (CSV, JSON, etc.)
or a database (SQLite, etc.). The persister is defined in .postit.toml, or you can
override it with the `-p` flag.

It can be used on the following commands:
- [sample](#sample)
- [view](#view)
- [add](#add)
- [check](#check)
- [uncheck](#uncheck)
- [drop](#drop)
- [clean](#clean)
- [remove](#remove)

There are currently 4 supported persisters:

- Files
  - csv (e.g.: tasks.csv)
  - json (e.g.: tasks.json)
  - xml (e.g.: tasks.xml)

- Databases
  - SQLite (e.g.: tasks.db, tasks.sqlite or tasks.sqlite3)

A use example:
```sh
postit view --persister tasks.csv 
```

## Development

### Testing

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
