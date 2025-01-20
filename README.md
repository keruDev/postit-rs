# 📝 postit-rs

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

A small command line program for managing tasks.

## Usage

The commands currently available are:
- `postit view`
- `postit add <TASK>`
- `postit check <IDS>`
- `postit uncheck <IDS>`
- `postit drop <IDS>`

You can also use `postit -h` for additional help.

The `-p` flag can be used on any command to specify the path of the file used
to manage tasks (default: `tasks.csv`).

## Features

`postit` is still in early development, so its features are currently limited.

Supported file formats:
- csv
- json

Display:
- Checked tasks appear as strikethrough.
- Different colors depending on priority.
  - `high`: red
  - `med`: yellow
  - `low`: blue
  - `none`: white

## Examples

Keep in mind every command uses `-p tasks.csv` by default.

Here is a sample of tasks so you test `postit`.

```csv
// tasks.csv

1,Task,low,false
2,Task,med,false
3,Task,high,true
4,Task,none,true
```

### view

Same as running just `postit`:

```csv
postit view

1,Task,low,false
2,Task,med,false
3,Task,high,true
4,Task,none,true
```

### add

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

Checks tasks if they are unchecked.

```csv
postit check 2,3

1,Task,low,false
2,Task,med,true         (changed)
3,Task,high,true        (not changed)
4,Task,none,true
```

### uncheck

Unchecks tasks if they are checked.

```csv
postit uncheck 2,3

1,Task,low,false
2,Task,med,false        (not changed)
3,Task,high,false       (changed)
4,Task,none,true
```

### drop

Note tasks must be checked to be dropped:

```csv
postit drop 2,3

1,Task,low,false
2,Task,med,false        (not dropped)
// 3,Task,high,true     (dropped)
4,Task,none,true
```

### copy

Copies a file's contents into another:

```sh
postit copy "tasks.csv" "tasks.json"
```
