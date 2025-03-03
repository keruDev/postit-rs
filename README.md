# 📝 postit-rs

[![Build Status](https://github.com/keruDev/postit-rs/workflows/CI/badge.svg)](https://github.com/keruDev/postit-rs/actions)
[![Current Crates.io Version](https://img.shields.io/crates/v/postit.svg)](https://crates.io/crates/postit)
[![Docs.rs](https://img.shields.io/badge/postit-blue.svg?label=docs.rs)](https://docs.rs/postit/latest/postit/)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

Postit is a CLI utility aimed to help you complete your tasks.
You can also save your tasks to keep track of them later.

## Features

Although `postit` is still in early development and it is limited in features,
it effectively serves its intended purpose.

Customization:
- Configuration file (more info in the [Configuration](#configuration) section).
- Set your own configuration path using the `POSTIT_CONFIG_PATH` environment variable 
  (by default and convention, `.postit.toml` or `postit.toml`).

Supported file formats:
- csv
- json

Display:
- Checked tasks appear crossed out.
- Different colors depending on priority.
  - `high`: red
  - `med`: yellow
  - `low`: blue
  - `none`: white

## Configuration

postit's behavior can be changed using the `.postit.toml` or `postit.toml` file
(the first one is preferred over the second one).

You can check out its possible fields in the [docs](https://docs.rs/postit/latest/postit/struct.Config.html) or down below:
- `path`: location of the default file where tasks are stored (the `-p` or `--path` flag can override this).
- `force_drop`: if true, allows dropping tasks without them being checked.
- `force_copy`: if true, allows overwriting files on copy if they already exist.
- `drop_after_copy`: if true, drops files after copying.


## Environment variables

- `POSTIT_CONFIG_PATH`: specifies where the `.postit.toml` or `postit.toml` is located.

## Usage

The commands currently available are:
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

Takes the `path` config defined at `postit.toml` (or the `-p` flag, if provided)
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

Adds a task with the format `id,content,priority,checked`:

- **id**: a unique unsigned integer.
- **content**: text contained.
- **priority**: `high`, `med`, `low` or `none`.
- **checked**: `true` or `false`.

```csv
postit add "5,New task,low,false"

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
- `edit`: opens the default editor to change configs.
- `drop`: deletes the config file (default values will be used at runtime).

You can also check the [Configuration](#configuration) section where each config
field is explained and there is a link to the official docs.

## Testing

To run postit's tests, use this command:
```sh
cargo test -- --test-threads=1
```

You can also use `tarpaulin`, configured in the `tarpaulin.toml` file.
It is slower, but shows line coverage (not branch coverage):
```sh
cargo tarpaulin
```

The reason why tests are run synchronously is to not overwrite existing files,
control the execution flow (creation and cleanup of temp files) and keep them
as lightweight as possible, as they don't use external dependencies.
