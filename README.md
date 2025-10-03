# simple-error

[![crates.io](https://img.shields.io/crates/v/simple-error.svg)](https://crates.io/crates/simple-error)
[![Documentation](https://docs.rs/simple-error/badge.svg)](https://docs.rs/simple-error/)
[![Build Status](https://github.com/wisagan/simple-error/workflows/ci/badge.svg)](https://github.com/WiSaGaN/simple-error/actions?query=workflow%3Aci)
[![Coverage Status](https://coveralls.io/repos/github/WiSaGaN/simple-error/badge.svg?branch=master)](https://coveralls.io/github/WiSaGaN/simple-error?branch=master)
[![MSRV](https://img.shields.io/badge/simple_error-rustc_1.58.0+-lightgray.svg)](https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html)

`simple-error` is a `Rust` library that provides a simple `Error` type backed by a `String`. It is best used when all you care about the error is an error string.

[Documentation](https://docs.rs/simple-error/)

## Usage

To use `simple-error`, first add this to your `Cargo.toml`:

```toml
[dependencies]
simple-error = "0.3"
```

Then import what you use (Rust 2018/2021):

```rust
use simple_error::{SimpleError, try_with};
// Add others as needed: require_with, ensure_with, bail, simple_error, map_err_with
```

Now you can use `simple-error` in different ways:

You can use it simply as a string error type:

```rust
fn do_foo() -> Result<(), SimpleError> {
    Err(SimpleError::new("cannot do foo"))
}
```

You can use it to replace all error types if you only care about a string description:

```rust
fn do_bar() -> Result<(), SimpleError> {
    Err(SimpleError::from(std::io::Error::new(std::io::ErrorKind::Other, "oh no")))
}
```

Or you can chain all the errors, and get a complete error description at the top level:

```rust
fn find_tv_remote() -> Result<(), SimpleError> {
    try_with!(std::fs::File::open("remotefile"), "failed to open remote file");
    Ok(())
}

fn turn_on_tv() -> Result<(), std::io::Error> {
    Ok(())
}

fn watch_tv() -> Result<(), SimpleError> {
    try_with!(find_tv_remote(), "tv remote not found");
    try_with!(turn_on_tv(), "cannot turn on tv");
    Ok(())
}

fn study() -> Result<(), SimpleError> {
    Ok(())
}

fn run() -> Result<(), SimpleError> {
    try_with!(study(), "cannot study");
    try_with!(watch_tv(), "cannot watch tv");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
// This prints out "cannot watch tv, tv remote not found, failed to open remote file, Text file busy" if the error is text file busy.
```

You can also ensure a condition holds and early-return a `SimpleError` if it does not:

```rust
use simple_error::{SimpleError, ensure_with};

fn check_config(is_valid: bool) -> Result<(), SimpleError> {
    ensure_with!(is_valid, "invalid config");
    Ok(())
}
```

## Macros

- `try_with!` — unwrap `Result`, else return `SimpleError::with`.
- `require_with!` — unwrap `Option`, else return `SimpleError::new`.
- `ensure_with!` — assert boolean, else return `SimpleError::new`.
- `bail!` — return early with a `SimpleError` (supports format/expr).
- `simple_error!` — construct a `SimpleError` (supports format/expr).
- `map_err_with!` — map a `Result`’s error with extra context.
