# Pom

A lightweight focus CLI tool built with Rust

```bash
pom 0.1.0
A lightweight focus tool inspired by pomodoro

USAGE:
    pom [OPTIONS]

OPTIONS:
    -b, --break-duration <BREAK_DURATION>    Break period duration (Minutes) [default: 5]
    -d, --duration <DURATION>                Focus period duration (Minutes) [default: 25]
    -h, --help                               Print help information
    -n, --name <NAME>                        Name of task [default: "a task"]
    -V, --version
```

## Build

Once you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed, clone this repository and go into it:

> cd pom

And install the executable:

> cargo install --path .
