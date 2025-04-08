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

You can also use the `example` command to know how to use every other command: 

```sh
postit example
```

Another useful command is `flag`, which you can use to get information about commonly used flags:
```sh
postit flag
```

`postit` uses a configuration file called `.postit.toml`.\
Use the `POSTIT_CONFIG_PATH` env var to tell `postit` where this file is:

On Linux:
```sh
# .bashrc executes operations before every shell session
nano ~/.bashrc

# Add this line to define POSTIT_CONFIG_PATH (feel free to change the path)
export POSTIT_CONFIG_PATH="$HOME/.postit.toml"
```  

On Windows:

```ps1
[Environment]::SetEnvironmentVariable("POSTIT_CONFIG_PATH", "$env:USERPROFILE\.postit.toml", "User")
```

After setting it, use the command below to generate the config structure:

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
- Commands and flags to manage tasks and files.
- Variety of supported file and database formats to persist data.
- Configuration file to change postit's behavior (more info in the [Configuration](#configuration) section).
- Tasks are displayed differently depending on their priority and whether they are checked or not.

Roadmap:
- [x] XML support
- [ ] MongoDB support
- [ ] MySQL support
- [ ] Tasks filtering and sorting

## Configuration

postit's behavior can be changed using the `.postit.toml` file.

You can check out its possible fields in the [docs](https://docs.rs/postit/latest/postit/struct.Config.html) or by running the example command:

```sh
postit example config
```

## Environment variables

Here is a list of the environment variables currently used:

- `EDITOR`: used to open your configuration file and edit it.
- `POSTIT_CONFIG_PATH`: where the config file is located (by default, `.postit.toml`).

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
