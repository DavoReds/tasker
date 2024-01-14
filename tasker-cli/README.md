# Tasker CLI

A command line application to manage your daily tasks.

## Installation

#### Crates

You can install Tasker CLI from the [crates.io](https://crates.io/) repository.

```bash
cargo install tasker-cli
```

I also recommend using a tool like
[cargo binstall](https://github.com/cargo-bins/cargo-binstall) for a faster
installation.

#### Binary Install

You can also download a precompiled binary from the
[releases](https://gitlab.com/DavoReds/tasker/-/releases) page. Currently only
Windows, Linux and x86 Mac's are present.

Select the file corresponding to your operating system and decompress it, you
can then put the binary on your PATH for ease of use.

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
      --todo-file <TO_DO_FILE>     Path to a file in which to look for and save Tasks
      --config-file <CONFIG_FILE>  Path to an alternative configuration file. Takes precedence over `todo-file`
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
