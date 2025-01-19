# 📝 postit-rs
Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

A small command line program for managing tasks.


## Usage

The commands currently available are:

- `postit -c view`
- `postit -c add -t <TASK>`
- `postit -c check -i <IDS>`
- `postit -c uncheck -i <IDS>`
- `postit -c drop -i <IDS>`

You can also use `postit -h` for additional help.

The `-p` flag can be used on any command to specify the path of the file used to manage tasks (default: `tasks.csv`).


## Features

`postit` is still in early development, so its features are currently limited.

Supported file formats:
- csv
- json

Display:
- Different colors depending on priority.
  - `high`: red
  - `med`: yellow
  - `low`: blue
  - `none`: white
- Checked tasks appear as strikethrough.


## Examples

Here is a sample with every possible task format so you can test `postit`.

```
// tasks.csv

1,Task,low,false
2,Task,med,false
3,Task,high,false
4,Task,none,false
5,Task,low,true
6,Task,med,true
7,Task,high,true
8,Task,none,true
```

Keep in mind every command uses `-p tasks.csv` by default.

### view

Same as running just `postit`:

```
postit -c view

1,Task,low,false
2,Task,med,false
3,Task,high,false
4,Task,none,false
5,Task,low,true
6,Task,med,true
7,Task,high,true
8,Task,none,true
```


### add

Adds a task with the format `id,content,priority,checked`:
- **id**: a unique unsigned integer.
- **content**: text contained.
- **priority**: `high`, `med`, `low` or `none`.
- **checked**: `true` or `false`.

```
postit -c add -t "9,New task,low,false"

1,Task,low,false
2,Task,med,false
3,Task,high,false
4,Task,none,false
5,Task,low,true
6,Task,med,true
7,Task,high,true
8,Task,none,true
9,New task,low,false    (new element)
```


### check

Checks tasks if they are unchecked.

```
postit -c check -i 3,4,5,6

1,Task,low,false
2,Task,med,false
3,Task,high,true        (changed)
4,Task,none,true        (changed)
5,Task,low,true         (not changed)
6,Task,med,true         (not changed)
7,Task,high,true
8,Task,none,true
```


### uncheck

Unchecks tasks if they are checked.

```
postit -c uncheck -i 3,4,5,6

1,Task,low,false
2,Task,med,false
3,Task,high,false       (not changed)
4,Task,none,false       (not changed)
5,Task,low,false        (changed)
6,Task,med,false        (changed)
7,Task,high,true
8,Task,none,true
```


### drop

Note tasks must be checked to be dropped:

```
postit -c drop -i 3,4,5,6

1,Task,low,false
2,Task,med,false
3,Task,high,false       (not dropped)
4,Task,none,false       (not dropped)
// 5,Task,low,true      (dropped)
// 6,Task,med,true      (dropped)
7,Task,high,true
8,Task,none,true
```
