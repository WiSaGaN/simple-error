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
