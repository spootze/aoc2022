# Advent of Code 2022 in Rust

This repo contains my solutions to the [2022 Advent of Code](https://adventofcode.com/2022). The solutions are written in Rust.

## Structure

The repo is organized into a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) where the solution to each day is implemented as a separate package.

## Running solutions

`cargo run -p dayNN` e.g., `cargo run -p day01`.

Examples are usually implemented as tests that can be executed with `cargo test -p dayNN`.
