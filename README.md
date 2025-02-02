# 📝 postit-rs

[![Current Crates.io Version](https://img.shields.io/crates/v/postit.svg)](https://crates.io/crates/postit)
[![Docs.rs](https://img.shields.io/badge/postit-blue.svg?label=docs.rs)](https://docs.rs/postit/latest/postit/)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

Postit is a CLI utility aimed to help you complete your tasks.
You can also save your tasks to keep track of them later.

## Features

`postit` is still in early development, so its features are currently limited.

Customization:
- Configuration file `postit.json` (more info in the [Configuration](#configuration) section).

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

postit's behavior can be changed using the `postit.json` file.

You can check out its possible fields in the [docs](https://docs.rs/postit/latest/postit/struct.Config.html) or down below:
- `path`: location of the default file where tasks are stored.
- `force_drop`: if true, allows dropping tasks without them being checked.
- `force_copy`: if true, allows overwriting files on copy if they already exist.
- `drop_after_copy`: if true, drops files after copying.

## Usage

The commands currently available are:
- [`view`](#view)
- [`add`](#add)
- [`check`](#check)
- [`uncheck`](#uncheck)
- [`drop`](#drop)
- [`copy`](#copy)

You can also use the `--help` flag for additional help on every command.

The `-p` or `--path` flag (default: `tasks.csv`) can be used on any command to
specify the path of the file used to manage tasks.

## Examples

Keep in mind every command uses `-p tasks.csv` by default.

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

Shows the list of current tasks:

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

Note tasks must be checked to be dropped:

```csv
postit drop 2,3

1,Task,low,false
2,Task,med,false        (not dropped)
// 3,Task,high,true     (dropped)
4,Task,none,true
```

### copy

Syntax: `postit copy <OLD_PATH> <NEW_PATH>`

Copies a file's contents into another:

```sh
postit copy "tasks.csv" "tasks.json"
```

