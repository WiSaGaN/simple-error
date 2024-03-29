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

Then add this to your crate root:

```rust
#[macro_use]
extern crate simple_error;

use simple_error::SimpleError;
```

Or you can skip the `extern crate` and just import relevant items you use if you are on 2018 edition or beyond.

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
    Err(SimpleError::from(std::io::Error(io::ErrorKind::Other, "oh no")))
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
