#[macro_use]
extern crate cpython;

use std::fmt;

use image::{Rgb, DynamicImage, ImageError};
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
    let src = image::open(&path)?.to_rgb();
    let mut dst = DynamicImage::new_rgb8(src.width(), src.height()).to_rgb();

    for (x, y, Rgb([r, g, b])) in src.enumerate_pixels() {
        let under = [*r, *g, *b].iter().any(|&x| x < cutoff);

        let pixel = if under {
            let out_r = *r - (*r as f32 * (amount as f32 / 100f32)) as u8;
            let out_g = *g - (*g as f32 * (amount as f32 / 100f32)) as u8;
            let out_b = *b - (*b as f32 * (amount as f32 / 100f32)) as u8;
            Rgb([out_r, out_g, out_b])
        } else {
            Rgb([*r, *g, *b])
        };

        dst.put_pixel(x, y, pixel);
    }

    dst.save("out.jpg").unwrap();

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
