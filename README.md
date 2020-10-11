# Paket 📦

> An experimental package manager for the [Fish shell](https://fishshell.com/). 🐠

This is an *experimental* Git-based *"package manager"* for Fish shell which is under **active** development.

## Features

- Rely only on [Git](https://git-scm.com/) binary and its well known features (clone, fetch, checkout, pull, tag, etc).
- No registries. Git repositories instead.
- Just uses the format: `username/package_name@(tag_name|branch_name)`
- Tiny but ultra fast static binary powered by [Rust](https://www.rust-lang.org/).
- Package file support (`paket.toml`) to

## Paket file 

`paket.toml` sample file of [GitNow](https://github.com/joseluisq/gitnow) Fish plugin.

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

include = [
    "conf.d/.gitnow"
]

[dependencies]
```

## Usage

```sh
~> paket --help
paket 0.0.0
A experimental package manager for the Fish shell

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

### Examples

```sh
~> paket add joseluisq/gitnow@2.4.0
~> paket up joseluisq/gitnow@2.5.0
~> paket rm joseluisq/gitnow
```

## TODO

Because its and an experimental repository some key functionalies are missing right now. So feel free to contribute.

- [x] Add command.
- [x] Update command.
- [x] Remove command.
- [x] Dispatch Fish shell `paket_install`, `paket_update`, `paket_uninstall` events.
- [x] Add package file `paket.toml` support.
- [ ] Add config file `~/paket.toml` support.
- [ ] Dependencies support.
- [ ] Ability to install package from local.
- [ ] Add support for Bitbucket, Gitlab, etc.
- [ ] Prevent unnecessary clones for same versions (tags)
- [ ] ?

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in current work by you, as defined in the Apache-2.0 license, shall be dual licensed as described below, without any additional terms or conditions.

Feel free to send some [Pull request](https://github.com/joseluisq/paket/pulls) or [issue](https://github.com/joseluisq/paket/issues).

## License

This work is primarily distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

© 2020-present [Jose Quintana](https://git.io/joseluisq)
