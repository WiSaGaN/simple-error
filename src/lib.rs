use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringError {
    err: String,
}

impl StringError {
    pub fn new<T: Into<String>>(t: T) -> StringError {
        StringError{ err: t.into() }
    }

    pub fn from<T: std::error::Error>(t: T) -> StringError {
        StringError{ err: format!("{}", t) }
    }
}

// TODO: implement From<T> where T: std::error::Error when specialization lands, and remove
// inherent from function.

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.err.fmt(f)
    }
}

impl std::error::Error for StringError {
    fn description(&self) -> &str {
        &self.err
    }
}

#[cfg(test)]
mod tests {
    use super::StringError;
    use std::error::Error;
    use std::io;

    #[test]
    fn new_from_string() {
        let err = StringError::new(String::from("an error from String"));
        assert_eq!("an error from String", format!("{}", err));
        assert_eq!("an error from String", err.description());
    }

    #[test]
    fn new_from_str() {
        let err = StringError::new("an error from str");
        assert_eq!("an error from str", format!("{}", err));
    }

    #[test]
    fn from_io_error() {
        let err = StringError::from(io::Error::new(io::ErrorKind::Other, "oh no"));
        assert_eq!("oh no", format!("{}", err));
    }
}
