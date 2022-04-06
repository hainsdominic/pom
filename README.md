# Pom

[![Current Crates.io Version](https://img.shields.io/crates/v/pom-rs.svg)](https://crates.io/crates/pom-rs)

A lightweight focus CLI tool built with Rust

```bash
pom-rs 0.1.X
A lightweight pomodoro focus tool with cross-platform desktop notifications on Linux, MacOS and
Windows.

USAGE:
    pom-rs [OPTIONS]

OPTIONS:
    -b, --break-duration <BREAK_DURATION>    Break period duration (Minutes) [default: 5]
    -d, --duration <DURATION>                Focus period duration (Minutes) [default: 25]
    -h, --help                               Print help information
    -n, --name <NAME>                        Name of task [default: "a task"]
    -V, --version
```

## Installation

Simple run `cargo install pom-rs` to install the CLI application, then use the `pom-rs` command to use it.

## Build

Once you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed, clone this repository and go into it:

> cd pom

And install the executable:

> cargo install --path .
