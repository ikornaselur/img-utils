use std::fmt;
use std::io;

use image::ImageError;

#[derive(Debug)]
pub enum ImgError {
    FileNotFound,
    IoError(io::Error),
    Error(ImageError),
}

impl fmt::Display for ImgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "img_utils error")
    }
}

impl From<ImageError> for ImgError {
    fn from(err: ImageError) -> ImgError {
        match err {
            ImageError::IoError(_) => ImgError::FileNotFound,
            _ => ImgError::Error(err),
        }
    }
}

impl From<io::Error> for ImgError {
    fn from(err: io::Error) -> ImgError {
        ImgError::IoError(err)
    }
}
