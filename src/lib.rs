#[macro_use]
extern crate cpython;

mod darken;
mod errors;
mod python;

pub use darken::darken_pixels;
pub use errors::ImgError;
