#[macro_use]
extern crate cpython;

use std::fmt;

use image::{Rgb, ImageError, RgbImage};
use cpython::{PyResult, Python, PyObject, PyErr, exc};

py_module_initializer!(img_utils, initimg_utils, PyInit_img_utils, |py, m| {
    m.add(py, "__doc__", "Image manipulation library")?;
    m.add(py, "increase_contrast", py_fn!(py, increase_contrast_py(src_path: String, dst_path: String, amount: u8, cutoff: u8)))?;
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


fn darken_pixels(image: RgbImage, amount: u8, cutoff: u8) -> Result<RgbImage, ImgError> {
    let mut dst = RgbImage::new(image.width(), image.height());

    let amount_frac = if amount > 100 {1.0f32} else {amount as f32 / 100f32};

    dst.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let Rgb([r, g, b]) = image.get_pixel(x, y);

        *pixel = if *r < cutoff || *g < cutoff || *b < cutoff {
            Rgb([
                *r - (*r as f32 * amount_frac) as u8,
                *g - (*g as f32 * amount_frac) as u8,
                *b - (*b as f32 * amount_frac) as u8,
            ])
        } else {
            Rgb([*r, *g, *b])
        };
    });


    Ok(dst)
}


pub fn increase_contrast(src_path: String, dst_path: String, amount: u8, cutoff: u8) -> Result<(), ImgError> {
    let src = image::open(&src_path)?.to_rgb();

    let dst = darken_pixels(src, amount, cutoff)?;
    dst.save(dst_path).unwrap();  // Convert to ImgError

    Ok(())
}



fn increase_contrast_py(python: Python, src_path: String, dst_path: String, amount: u8, cutoff: u8) -> PyResult<PyObject> {
    match increase_contrast(src_path, dst_path, amount, cutoff) {
        Ok(()) => Ok(Python::None(python)),
        Err(e) => {
            match e {
                ImgError::FileNotFound => Err(PyErr::new::<exc::RuntimeError, _>(python, "File not found")),
                e => Err(PyErr::new::<exc::RuntimeError, _>(python, format!("{:?}", e))),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn darken_pixels_only_affects_pixels_with_subpixels_below_cutoff() {
        let src: RgbImage = RgbImage::from_vec(3, 3, vec![
            00, 00, 00, 10, 10, 10, 20, 20, 20,
            30, 30, 30, 40, 40, 40, 50, 50, 50,
            60, 60, 60, 70, 70, 70, 80, 80, 80,
        ]).unwrap();

        let dst = darken_pixels(src, 100, 50).unwrap();

        assert_eq!(dst.into_vec(), vec![
            00, 00, 00, 00, 00, 00, 00, 00, 00,
            00, 00, 00, 00, 00, 00, 50, 50, 50,
            60, 60, 60, 70, 70, 70, 80, 80, 80,
        ]);
    }

    #[test]
    fn darken_pixels_affects_pixels_if_any_subpixel_below_cutoff() {
        let src: RgbImage = RgbImage::from_vec(3, 2, vec![
            50, 50, 50, 50, 50, 49, 50, 49, 50,
            49, 50, 50, 49, 49, 50, 49, 49, 49,
        ]).unwrap();

        let dst = darken_pixels(src, 100, 50).unwrap();

        assert_eq!(dst.into_vec(), vec![
            50, 50, 50, 00, 00, 00, 00, 00, 00,
            00, 00, 00, 00, 00, 00, 00, 00, 00,
        ]);
    }

    #[test]
    fn darken_pixels_darkens_by_given_amount_10() {
        let src: RgbImage = RgbImage::from_vec(1, 1, vec![80, 100, 200]).unwrap();

        let dst = darken_pixels(src, 10, 255).unwrap();

        assert_eq!(dst.into_vec(), vec![72, 90, 180]);
    }

    #[test]
    fn darken_pixels_darkens_by_given_amount_50() {
        let src: RgbImage = RgbImage::from_vec(1, 1, vec![80, 100, 200]).unwrap();

        let dst = darken_pixels(src, 50, 255).unwrap();

        assert_eq!(dst.into_vec(), vec![40, 50, 100]);
    }

    #[test]
    fn darken_pixels_handles_amount_above_100() {
        let src: RgbImage = RgbImage::from_vec(1, 1, vec![80, 100, 200]).unwrap();

        let dst = darken_pixels(src, 255, 255).unwrap();

        assert_eq!(dst.into_vec(), vec![0, 0, 0]);
    }
}
