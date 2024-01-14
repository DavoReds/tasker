[![CI status](https://gitlab.com/DavoReds/tasker-cli/badges/main/pipeline.svg)](https://gitlab.com/DavoReds/tasker-cli/-/commits/main)
[![Latest Release](https://gitlab.com/DavoReds/tasker-cli/-/badges/release.svg)](https://gitlab.com/DavoReds/tasker-cli/-/releases)

# Tasker

A series of cross-platform applications to manage your daily tasks. This
includes a command line application and a graphical application. All written in
Rust.

All applications encode tasks as a [RON](https://github.com/ron-rs/ron) file and
are interoperable between them.

## Features

- Cross platform
- Use of text files instead of an internal database
- No internet connection required
- Written in Rust, btw

## Installation

### Tasker CLI

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
[releases](https://github.com/DavoReds/tasker/releases) page. Currently only
Windows, Linux and x86 Mac's are present.

Select the file corresponding to your operating system and decompress it, you
can then put the binary on your PATH for ease of use.

## Roadmap

- Develop Tasker GUI

## Contributing

Contributions are always welcome!

## License

[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)
