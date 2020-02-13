# SMarkov

A small Markov Chain generator. This is a just-for-fun project I made to try out Rust.

## Disclaimer

Sorry, no docs nor tests. Use this thing at your own risk!

# Development setup

Install a rust environment using the instructions found at [https://rustup.rs/].

## Building and Running

The simplest way in a development environment is to use `cargo` for building/running, e.g. to build/run a debug build:
```
cargo build
cargo run
```

An example input-file can be found in the `/example_input` sub-directory:
```
cargo run -- example_input/poem.txt
```

## Usage through CLI

```
Usage: smarkov <inputfile> [OPTIONS]

Description:
  Processes an UTF-8 encoded text file and outputs a Markov Chain matrix on JSON format.

Arguments:
  <inputfile>   An UTF-8 coded text file.
Options:
  --minimize    Produces minimized JSON output
```
