#[macro_use]
extern crate cpython;

mod darken;
mod errors;
mod extract;
mod python;

pub use darken::darken_pixels;
pub use errors::ImgError;
pub use extract::extract_blues;
