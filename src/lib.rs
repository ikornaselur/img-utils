#[macro_use]
extern crate cpython;

mod darken;
mod darken_py;

pub use darken::{darken_pixels, ImgError};
