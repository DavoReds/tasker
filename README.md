![Crates.io License](https://img.shields.io/crates/l/tasker-cli?style=flat-square&logo=rust&color=%2374c7ec)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/DavoReds/tasker/ci.yml?branch=main&style=flat-square&logo=github&label=CI&color=%23a6e3a1)
![GitHub Release](https://img.shields.io/github/v/release/DavoReds/tasker?sort=semver&display_name=release&style=flat-square&logo=github&color=%23f38ba8)

# Tasker

A series of cross-platform applications to manage your daily tasks. This includes a command line application and a graphical application. All written in Rust.

All applications encode tasks as a [RON](https://github.com/ron-rs/ron) file and are interoperable between them.

## Features

- Create any number of tasks with optional projects and tags.
- Manage the state of each task between to-do, doing and done.
- Clean completed tasks.
- Written in Rust, btw.

## Installation

### Tasker CLI

#### Crates

You can install Tasker CLI from [crates.io](https://crates.io/).

```sh
cargo install tasker-cli
```

I also recommend using a tool like [cargo binstall](https://github.com/cargo-bins/cargo-binstall) for a faster installation.

```sh
cargo binstall tasker-cli
```

#### Binary Install

You can also download a precompiled binary from the [releases](https://github.com/DavoReds/tasker/releases) page.

## Roadmap

- [ ] Develop Tasker GUI

## Contributing

Contributions are always welcome!

You're going to need to have [Rust](https://www.rust-lang.org/) installed. I also recommend having [Bacon](https://dystroy.org/bacon/) (to handle recompilation) and [cargo-nextest](https://nexte.st/) (for a nicer interface when running tests) installed.

We also try to [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/) conventions when contributing to the repo.

## License

[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)
