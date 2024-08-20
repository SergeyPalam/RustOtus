use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub struct Err {
    err: String,
}

impl Err {
    pub fn new(err: &str) -> Self {
        Self {
            err: err.to_owned(),
        }
    }
}

impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.err)
    }
}

impl From<io::Error> for Err {
    fn from(value: io::Error) -> Self {
        Self::new(&value.to_string())
    }
}
