// src/errors.rs
use std::io;

#[derive(Debug)]
pub enum ProcessingError {
    IoError { file: String, source: io::Error },
    EncodingError { file: String },
    Cancelled,
}

impl From<(String, io::Error)> for ProcessingError {
    fn from((file, err): (String, io::Error)) -> Self {
        ProcessingError::IoError { file, source: err }
    }
}
