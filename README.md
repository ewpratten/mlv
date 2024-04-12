# Mini Log Viewer
[![Crates.io](https://img.shields.io/crates/v/mini-log-viewer)](https://crates.io/crates/mini-log-viewer)
[![Build](https://github.com/Ewpratten/mlv/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/mlv/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/mlv/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/mlv/actions/workflows/clippy.yml)

`mlv` is a small log viewer application that works with files and streams.

![A screenshot of MLV in use](./screenshot.png)

```sh
# Reading a file
mlv /path/to/file.log

# Reading Journalctl output
mlv -p journal-json <(journalctl -o json -f)

# Reading from a remote machine
mlv <(ssh user@host "cat /path/to/remote.log")
```

## Installation

This crate can be installed via `cargo` with:

```sh
cargo install mini-log-viewer
```

Don't forget, the installed binary is called `mlv`.