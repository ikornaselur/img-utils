#[macro_use]
extern crate cpython;

use std::fmt;

use image::{Rgb, ImageError};
use cpython::{PyResult, Python, PyErr, exc};

py_module_initializer!(img_utils, initimg_utils, PyInit_img_utils, |py, m| {
    m.add(py, "__doc__", "Image manipulation library")?;
    m.add(py, "increase_contrast", py_fn!(py, increase_contrast_py(path: String, amount: u8, cutoff: u8)))?;
    Ok(())
});

#[derive(Debug)]
pub enum ImgError {
    FileNotFound,
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


pub fn increase_contrast(path: String, amount: u8, cutoff: u8) -> Result<String, ImgError> {
    let mut img = image::open(&path)?.to_rgb();

    for (_, _, pixel) in img.enumerate_pixels_mut() {
        let Rgb([r, g, b]) = pixel;
        let lower = 0;

        *pixel = if lower < cutoff {
            let new_r = *r - ((*r as f32) * (amount as f32 / 100f32)) as u8;
            let new_g = *g - ((*g as f32) * (amount as f32 / 100f32)) as u8;
            let new_b = *b - ((*b as f32) * (amount as f32 / 100f32)) as u8;
            Rgb([new_r, new_g, new_b])
        } else {
            Rgb([*r, *g, *b])
        };
    }

    img.save("out.jpg").unwrap();

    Ok(path)
}

fn increase_contrast_py(python: Python, path: String, amount: u8, cutoff: u8) -> PyResult<String> {
    match increase_contrast(path, amount, cutoff) {
        Ok(out) => Ok(out),
        Err(e) => {
            match e {
                ImgError::FileNotFound => Err(PyErr::new::<exc::RuntimeError, _>(python, "File not found")),
                e => Err(PyErr::new::<exc::RuntimeError, _>(python, format!("{:?}", e))),
            }
        }
    }
}
