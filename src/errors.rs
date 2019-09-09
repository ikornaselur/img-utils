use std::fmt;
use std::io;

use cpython::{exc, PyErr, Python};
use image::ImageError;

#[derive(Debug)]
pub enum ImgError {
    IoError(io::Error),
    Error(ImageError),
}

py_exception!(exceptions, ImgUtilsException);
py_exception!(exceptions, FileNotFoundException, ImgUtilsException);
py_exception!(exceptions, UnsupportedFormatException, ImgUtilsException);

impl ImgError {
    pub fn to_pyerr(&self, py: Python) -> PyErr {
        match *self {
            ImgError::IoError(ref e) => {
                PyErr::new::<FileNotFoundException, _>(py, format!("{}", e))
            }
            ImgError::Error(ref e) => match e {
                ImageError::UnsupportedError(_) => {
                    PyErr::new::<UnsupportedFormatException, _>(py, format!("{}", e))
                }
                _ => PyErr::new::<exc::RuntimeError, _>(py, format!("{}", e)),
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
