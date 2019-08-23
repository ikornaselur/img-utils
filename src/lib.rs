#[macro_use]
extern crate cpython;

use image::Rgb;
use cpython::{PyResult, Python};

py_module_initializer!(img_utils, initimg_utils, PyInit_img_utils, |py, m| {
    m.add(py, "__doc__", "Image manipulation library")?;
    m.add(py, "increase_contrast", py_fn!(py, increase_contrast_py(path: String, amount: u8, cutoff: u8)))?;
    Ok(())
});

pub fn increase_contrast(path: String, amount: u8, cutoff: u8) -> String {
    let mut img = image::open(&path).unwrap().to_rgb();

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

    path
}

fn increase_contrast_py(_: Python, path: String, amount: u8, cutoff: u8) -> PyResult<String> {
    let out = increase_contrast(path, amount, cutoff);
    Ok(out)
}
