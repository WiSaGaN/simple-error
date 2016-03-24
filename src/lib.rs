#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/")]
//! A simple error type backed by a string.
//!
//! This crate provides a `SimpleError` type, which implements `std::error::Error`. The underlying
//! is a `String` as the error message.
//!
//! It should be used when all you care about is an error string.
//!
//! It should not be used when you want to programmatically handle the error.

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleError {
    err: String,
}

impl SimpleError {
    pub fn new<T: Into<String>>(t: T) -> SimpleError {
        SimpleError{ err: t.into() }
    }

    pub fn from<T: std::error::Error>(t: T) -> SimpleError {
        SimpleError{ err: format!("{}", t) }
    }
}

// TODO: implement From<T> where T: std::error::Error when specialization lands, and remove
// inherent from function.

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.err.fmt(f)
    }
}

impl std::error::Error for SimpleError {
    fn description(&self) -> &str {
        &self.err
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleError;
    use std::error::Error;
    use std::io;

    #[test]
    fn new_from_string() {
        let err = SimpleError::new(String::from("an error from String"));
        assert_eq!("an error from String", format!("{}", err));
        assert_eq!("an error from String", err.description());
    }

    #[test]
    fn new_from_str() {
        let err = SimpleError::new("an error from str");
        assert_eq!("an error from str", format!("{}", err));
    }

    #[test]
    fn from_io_error() {
        let err = SimpleError::from(io::Error::new(io::ErrorKind::Other, "oh no"));
        assert_eq!("oh no", format!("{}", err));
    }
}
