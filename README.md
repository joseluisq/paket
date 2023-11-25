# Paket 📦 [![devel](https://github.com/joseluisq/paket/actions/workflows/devel.yml/badge.svg)](https://github.com/joseluisq/paket/actions/workflows/devel.yml) [![Docker Image Version (tag latest semver)](https://img.shields.io/docker/v/joseluisq/paket/latest)](https://hub.docker.com/r/joseluisq/paket/) [![Docker Image Size (tag)](https://img.shields.io/docker/image-size/joseluisq/paket/latest)](https://hub.docker.com/r/joseluisq/paket/) [![Docker Image](https://img.shields.io/docker/pulls/joseluisq/paket.svg)](https://hub.docker.com/r/joseluisq/paket/)

> A simple and fast package manager for the [Fish Shell](https://fishshell.com/) written in [Rust](https://www.rust-lang.org/). 🐠

This is a Git-based *"package manager"* for [Fish Shell](https://fishshell.com/) which is under **active** development.

**Note:** Some features are missing right now but it covers the most of functionalities needed to be usable. However, feel free to contribute. See [TODO](#todo) list.

## Features

- Rely only on [Git](https://git-scm.com/) binary and its well-known features (clone, fetch, checkout, pull, tag, etc).
- No registries. Git repositories instead.
- Just uses the format: `username/package_name@(tag_name|branch_name)`.
- Install, update or remove packages from local repositories or remote ones.
- Tiny but ultra-fast static binary powered by [Rust](https://www.rust-lang.org/).
- Package file support ([`paket.toml`](#package-file)) to describe a package and copy optional non `.fish` files.
- Trigger [Fish shell events](https://fishshell.com/docs/current/cmds/emit.html) when a package is installed, updated or uninstalled.
- It runs only on top of a Fish shell session (Fish parent process).
- First-class Docker support.

## Download/Install

- Docker image using latest Alpine → [hub.docker.com/r/joseluisq/paket](https://hub.docker.com/r/joseluisq/paket)
- Pre-compiled binaries for Linux, macOS and other targets → [github.com/joseluisq/paket/releases](https://github.com/joseluisq/paket/releases)

### MacOS

Via [Homebrew](https://brew.sh/) (also Linux)


```sh
brew tap joseluisq/paket

# Just the binary
brew install paket-bin

# Or build from source
brew install paket-src
```

## Usage

```sh
~> paket --help
paket 0.1.3
Jose Quintana <https://joseluisq.net>
A simple and fast package manager for the Fish shell

USAGE:
    paket [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add     Install a new package from a local or remote repository
    help    Prints this message or the help of the given subcommand(s)
    rm      Uninstall an existing package from a local or remote repository
    up      Update an existing package from a local or remote repository
```

### Examples

#### Remote packages

```sh
~> paket add joseluisq/gitnow@2.4.0
~> paket up joseluisq/gitnow@2.5.1
~> paket rm joseluisq/gitnow
```

#### Local packages

```sh
~> paket add ~/some-dir/my-pckage-dir
~> paket up ~/some-dir/my-pckage-dir
~> paket rm ~/some-dir/my-pckage-dir
```

## Paket file

Paket supports a `paket.toml` file to describe a package and copy optional non `.fish` files.
Here is a sample file of [GitNow](https://github.com/joseluisq/gitnow) Fish package.

```toml
[package]
name = "gitnow"
version = "2.5.1"
authors = ["Jose Quintana <https://joseluisq.net>"]
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

# Copy extra non Fish files (optional)
include = [
    "conf.d/.gitnow"
]

# Paket events which can trigger Fish shell events (optional)
# Use the format `[package_name]_[event_name]` without the brackets and spaces (underscores instead).
# Also make sure that every value match with your package's Fish shell event (--on-event).
[events]
after-install = "gitnow_install"
after-update = "gitnow_update"
before-uninstall = "gitnow_uninstall"

[dependencies]
```

## Fish shell events

Paket has a few events which can be used to trigger [Fish shell events](https://fishshell.com/docs/current/cmds/emit.html) defined in your package.

- `after-install`: After a package is installed.
- `after-update`: After a package is updated.
- `before-uninstall`: Before a package is uninstalled.

### Events definition and format

Appending a `-e` or `--on-event` flag to your function(s) tells Fish to run it when the specified named event is emitted.

Use the format `[package_name]_[event_name]` without the brackets and spaces (underscores instead).
Also, make sure that every value matches with your package's Fish Shell event (`--on-event`).

### Package example

Define a `paket.toml` file for your package:

```toml
[package]
name = "mypackage"
version = "0.0.0"
# ...

# Events (optional)
[events]
after-install = "mypackage_install"
before-uninstall = "mypackage_uninstall"
```

Optionally you can sign your corresponding functions with the `--on-event` (`-e`) value defined above.

```fish
# It will be immediately run after the package is installed
function __my_package_install -e mypackage_install
    echo "Installing my package..."
end

# It will be immediately run before the package is uninstalled
function __my_package_uninstall -e mypackage_uninstall
    echo "Uninstalling my package..."
end
```

Find an example on [GitNow](https://github.com/joseluisq/gitnow/blob/master/conf.d/gitnow.fish) repository.

## TODO

- [x] Add command.
- [x] Update command.
- [x] Remove command.
- [x] Add package file `paket.toml` support which describes a package.
- [x] Add support for Bitbucket, Gitlab and Github (default).
- [x] Dispatch Fish shell events when a package is installed, updated or uninstalled.
- [x] Define Paket events via `paket.toml` file.
- [x] Ability to install, update and remove packages from local repositories.
- [ ] Prevent unnecessary clones for the same versions (branches/tags)
- [ ] Dependencies support.
- [ ] Add configuration file `~/paket.toml` support.
- [ ] ?

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in current work by you, as defined in the Apache-2.0 license, shall be dual licensed as described below, without any additional terms or conditions.

Feel free to send some [Pull request](https://github.com/joseluisq/paket/pulls) or [issue](https://github.com/joseluisq/paket/issues).

## License

This work is primarily distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

© 2020-present [Jose Quintana](https://joseluisq.net)
