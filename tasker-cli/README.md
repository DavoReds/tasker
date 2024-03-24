# Tasker CLI

A command line application to manage your daily tasks.

## Installation

#### Crates.io

You can install Tasker CLI from the [crates.io](https://crates.io/) registry.

```bash
cargo install tasker-cli
```

I also recommend using a tool like
[cargo binstall](https://github.com/cargo-bins/cargo-binstall) for a faster
installation.

#### Binary Install

You can also download a precompiled binary from the
[releases](https://gitlab.com/DavoReds/tasker/-/releases) page. Currently only
Windows and Linux binaries are present.

Select the one that corresponds to your operating system and extract it, you can
then move the binary to somewhere on your PATH for ease of use.

#### Build From Source

To build this package from source, you'll the need the
[Rust Toolchain](https://www.rust-lang.org/learn/get-started) installed.

1. Clone this repository.
2. Execute `cargo build --release`
3. The binary will be built in the `target/release` directory at the root of the
   repository.

I also recommend using a tool like [UPX](https://upx.github.io/) to compress the
resulting binary and [GNU Stow](https://www.gnu.org/software/stow/) to manage
symlinks and avoid managing the package manually.

## Usage

```bash
$ tasker-cli help

Usage: tasker-cli [OPTIONS] [COMMAND]

Commands:
  add     Add one Task [aliases: a]
  addm    Add multiple Tasks [aliases: am]
  clean   Clean completed Tasks [aliases: c]
  delete  Delete Tasks [aliases: d]
  edit    Edit a Task [aliases: e]
  list    List Tasks [aliases: l]
  paths   Print default paths for the application [aliases: p]
  toggle  Change the state of a Task [aliases: t]
  help    Print this message or the help of the given subcommand(s)

Options:
  -T, --todo-file <TODO_FILE>      Path to a file in which to look for and save Tasks
  -C, --config-file <CONFIG_FILE>  Path to an alternative configuration file. Takes precedence over `todo-file`
  -h, --help                       Print help
  -V, --version                    Print version
```

## Examples

### Creating a Task

[![asciicast](https://asciinema.org/a/623078.svg)](https://asciinema.org/a/623078)

### Creating multiple Tasks

[![asciicast](https://asciinema.org/a/623079.svg)](https://asciinema.org/a/623079)

### Changing the state of a Task

[![asciicast](https://asciinema.org/a/623082.svg)](https://asciinema.org/a/623082)

### Cleaning completed Tasks

[![asciicast](https://asciinema.org/a/623084.svg)](https://asciinema.org/a/623084)

### Deleting a Task

[![asciicast](https://asciinema.org/a/623088.svg)](https://asciinema.org/a/623088)

### Editing a Task

[![asciicast](https://asciinema.org/a/623091.svg)](https://asciinema.org/a/623091)

### Listing Tasks

[![asciicast](https://asciinema.org/a/623100.svg)](https://asciinema.org/a/623100)

## License

[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)
