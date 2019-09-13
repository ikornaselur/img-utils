use cpython::{PyModule, PyObject, PyResult, Python};

use crate::{darken_pixels, errors, extract_blues};

py_module_initializer!(img_utils, initimg_utils, PyInit_img_utils, |py, m| {
    m.add(py, "__doc__", "Image manipulation library")?;
    m.add(py, "__package__", "img_utils.img_utils")?;
    m.add(
        py,
        "_darken_pixels",
        py_fn!(
            py,
            darken_pixels_py(src_path: String, dst_path: String, amount: u8, cutoff: u8)
        ),
    )?;
    m.add(
        py,
        "_extract_blues",
        py_fn!(
            py,
            extract_blues_py(
                src_path: String,
                dst_path: String,
                min_diff: u8,
                min_blue: u8
            )
        ),
    )?;
    // Exceptions
    let exceptions = PyModule::new(py, "exceptions")?;
    exceptions.add(py, "__package__", "img_utils.img_utils.exceptions")?;
    exceptions.add(
        py,
        "ImgUtilsException",
        py.get_type::<errors::ImgUtilsException>(),
    )?;
    exceptions.add(
        py,
        "FileNotFoundException",
        py.get_type::<errors::FileNotFoundException>(),
    )?;
    exceptions.add(
        py,
        "UnsupportedFormatException",
        py.get_type::<errors::UnsupportedFormatException>(),
    )?;

    m.add(py, "exceptions", exceptions)?;
    Ok(())
});

fn darken_pixels_py(
    python: Python,
    src_path: String,
    dst_path: String,
    amount: u8,
    cutoff: u8,
) -> PyResult<PyObject> {
    match darken_pixels(src_path, dst_path, amount, cutoff) {
        Ok(()) => Ok(Python::None(python)),
        Err(e) => Err(e.to_pyerr(python)),
    }
}

fn extract_blues_py(
    python: Python,
    src_path: String,
    dst_path: String,
    min_diff: u8,
    min_blue: u8,
) -> PyResult<PyObject> {
    match extract_blues(src_path, dst_path, min_diff, min_blue) {
        Ok(()) => Ok(Python::None(python)),
        Err(e) => Err(e.to_pyerr(python)),
    }
}
