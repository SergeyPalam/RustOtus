use std::fmt::Display;
use std::io;
use serde_json;

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    IoTimeOut,
    IoError,
    ParsingError,
    ServiceNotRespond,
    UnknownService,
    SerializationError,
    UnknownTypePack,
    WrongDevType,
}

#[derive(Debug)]
pub struct Err {
    err_kind: ErrorKind,
}

impl Err {
    pub fn new(err_kind: ErrorKind) -> Self {
        Self {
            err_kind,
        }
    }
    pub fn kind(&self) -> ErrorKind {
        self.err_kind
    }
}

impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.err_kind))
    }
}

impl From<io::Error> for Err {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::TimedOut => {
                Self::new(ErrorKind::IoTimeOut)
            }
            _ => {
                Self::new(ErrorKind::IoError)
            }
        }
    }
}

impl From<serde_json::Error> for Err {
    fn from(_: serde_json::Error) -> Self {
        Self::new(ErrorKind::ParsingError)
    }
}
