use std::fmt;
use std::io;

use cpython::{exc, PyErr, Python};
use image::ImageError;

#[derive(Debug)]
pub enum ImgError {
    IoError(io::Error),
    Error(ImageError),
}

impl ImgError {
    pub fn to_pyerr(&self, py: Python) -> PyErr {
        match *self {
            ImgError::IoError(ref e) => match e.kind() {
                std::io::ErrorKind::NotFound => PyErr::new::<exc::RuntimeError, _>(py, 1001),
                _ => PyErr::new::<exc::RuntimeError, _>(py, 9001),
            },
            ImgError::Error(ref e) => match e {
                ImageError::UnsupportedError(_) => PyErr::new::<exc::RuntimeError, _>(py, 1002),
                _ => PyErr::new::<exc::RuntimeError, _>(py, 9002),
            },
        }
    }
}

impl fmt::Display for ImgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "img_utils error")
    }
}

impl From<ImageError> for ImgError {
    fn from(err: ImageError) -> ImgError {
        match err {
            ImageError::IoError(err) => ImgError::IoError(err),
            _ => ImgError::Error(err),
        }
    }
}

impl From<io::Error> for ImgError {
    fn from(err: io::Error) -> ImgError {
        ImgError::IoError(err)
    }
}
