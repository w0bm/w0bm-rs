# w0bm v2 [![Build Status](https://travis-ci.org/w0bm/w0bm-rs.svg?branch=master)](https://travis-ci.org/w0bm/w0bm-rs)

Experimental backend for [w0bm](https://w0bm.com)

## Building

You need to have rustc and cargo nightly.

Install by running:  
`curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly`

1. `cargo install diesel_cli`
2. Set up `.env` or set `DATABASE_ENV` env variable
3. `diesel setup`
4. `cargo build`

## Running

`cargo run` or `cargo run --release`  
or  
`cargo build --release && target/release/w0bm-rs`
