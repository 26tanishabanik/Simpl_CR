use std::{fmt::Display, io::{Write, stdout}};

#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ERROR: {}", self.msg)
    }
}