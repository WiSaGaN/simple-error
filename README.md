# simple-error

[![crates.io](http://meritbadge.herokuapp.com/simple-error)](https://crates.io/crates/simple-error)
[![Build Status](https://travis-ci.org/WiSaGaN/simple-error.svg?branch=master)](https://travis-ci.org/WiSaGaN/simple-error)
[![Coverage Status](https://coveralls.io/repos/github/WiSaGaN/simple-error/badge.svg?branch=master)](https://coveralls.io/github/WiSaGaN/simple-error?branch=master)

`simple-error` is a `Rust` library that provides a simple `Error` type which is backed by a `String`. It is best used when all you care about the error is an error string.

[Documentation](https://wisagan.github.io/simple-error/simple_error/)

## Usage

To use `simple-error`, first add this to your `Cargo.toml`:

```toml
[dependencies]
simple-error = "0.1"
```

Then, add this to your crate root:

```rust
extern crate simple_error;
```
