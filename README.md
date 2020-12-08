# Paket 📦 [![Build Status](https://travis-ci.com/joseluisq/paket.svg?branch=master)](https://travis-ci.com/joseluisq/paket) [![Docker Image Version (tag latest semver)](https://img.shields.io/docker/v/joseluisq/paket/latest)](https://hub.docker.com/r/joseluisq/paket/) [![Docker Image Size (tag)](https://img.shields.io/docker/image-size/joseluisq/paket/latest)](https://hub.docker.com/r/joseluisq/paket/) [![Docker Image](https://img.shields.io/docker/pulls/joseluisq/paket.svg)](https://hub.docker.com/r/joseluisq/paket/)

> A simple and fast package manager for the [Fish shell](https://fishshell.com/) written in [Rust](https://www.rust-lang.org/). 🐠

This is an *WIP* Git-based *"package manager"* for Fish shell which is under **active** development.

## Features

- Rely only on [Git](https://git-scm.com/) binary and its well known features (clone, fetch, checkout, pull, tag, etc).
- No registries. Git repositories instead.
- Just uses the format: `username/package_name@(tag_name|branch_name)`
- Tiny but ultra fast static binary powered by [Rust](https://www.rust-lang.org/).
- Package file support ([`paket.toml`](#package-file)) to describe a package and copy optional non `.fish` files.
- Trigger [Fish shell events](https://fishshell.com/docs/current/cmds/emit.html) when a package is installed, updated or uninstalled.
- It runs only on top of a Fish shell session (Fish parent process).
- Docker support.

## Install

- Docker Alpine 3.11 image: [hub.docker.com/r/joseluisq/paket](https://hub.docker.com/r/joseluisq/paket)
- Release binaries for Linux/Macos amd64: [github.com/joseluisq/paket/releases](https://github.com/joseluisq/paket/releases)

## Usage

```sh
~> paket --help
paket 0.0.0
A simple package manager for the Fish shell 📦

USAGE:
    paket [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add     Add a new package from a local or remote repository
    help    Prints this message or the help of the given subcommand(s)
    rm      Remove an existing package
    up      Update an existing package
```

#### Examples

```sh
~> paket add joseluisq/gitnow@2.4.0
~> paket up joseluisq/gitnow@2.5.1
~> paket rm joseluisq/gitnow
```

## Paket file

Paket supports a `paket.toml` file in order to describe a package and copy optional non `.fish` files.
Here a sample file of [GitNow](https://github.com/joseluisq/gitnow) Fish package.

```toml
[package]
name = "gitnow"
version = "2.5.1"
authors = ["Jose Quintana <git.io/joseluisq>"]
license = "MIT OR Apache-2.0"
description = "Speed up your Git workflow. 🐠"
repository = "https://github.com/joseluisq/gitnow"
keywords = [
    "git",
    "git-workflow",
    "fish-shell",
    "fish-packages",
    "keybindings",
    "bitbucket",
    "github"
]

# Copy extra non Fish files
include = [
    "conf.d/.gitnow"
]

[dependencies]
```

## Fish shell events

Since Fish shell supports [events](https://fishshell.com/docs/current/cmds/emit.html), Paket will emit the following ones:

- `paket_install`: After a package gets installed.
- `paket_update`: After a package gets updated.
- `paket_uninstall`: After a package gets uninstalled.

### Events definition

Just appending a `-e` (`--on-event`) flag to your function(s) tells Fish to run it when the specified named event gets emitted.

Examples:

```fish
# It will be immediately run after the package is installed
function __my_package_install -e paket_install
    echo "Installing my package..."
end

# It will be immediately run after the package is uninstalled
function __my_package_uninstall -e paket_uninstall
    echo "Uninstalling my package..."
end
```

Find out a detailed example on [GitNow](https://github.com/joseluisq/gitnow/blob/master/conf.d/gitnow.fish) repository.

## TODO

Because its a WIP repository some functionalies are missing right now. So feel free to contribute.

- [x] Add command.
- [x] Update command.
- [x] Remove command.
- [x] Dispatch Fish shell `paket_install`, `paket_update`, `paket_uninstall` events.
- [x] Add package file `paket.toml` support.
- [ ] Dependencies support.
- [ ] Prevent unnecessary clones for same versions (tags)
- [ ] Ability to install package from local.
- [ ] Add configuration file `~/paket.toml` support.
- [ ] Add support for Bitbucket, Gitlab, etc. Github for now.
- [ ] ?

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in current work by you, as defined in the Apache-2.0 license, shall be dual licensed as described below, without any additional terms or conditions.

Feel free to send some [Pull request](https://github.com/joseluisq/paket/pulls) or [issue](https://github.com/joseluisq/paket/issues).

## License

This work is primarily distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

© 2020-present [Jose Quintana](https://git.io/joseluisq)
