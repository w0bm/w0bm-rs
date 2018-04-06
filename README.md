# w0bm v2 [![Build Status](https://travis-ci.org/w0bm/w0bm-rs.svg?branch=master)](https://travis-ci.org/w0bm/w0bm-rs)

Experimental backend for w0bm

## Building

You need to have rustc and cargo nightly.

Install via `curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly`

1. `cargo install diesel_cli`
2. Set up .env or set `DATABASE_ENV` env variable
3. `diesel setup`
4. `cargo build`

## Running

`cargo run`