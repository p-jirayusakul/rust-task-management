use std::fmt;

#[derive(Debug)]
pub struct MyError {
    pub message: String,
}

impl MyError {
    pub(crate) fn new(p0: String) -> Self {
        MyError { message: p0 }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError occurred: {}", self.message)
    }
}

impl std::error::Error for MyError {}
