use cpython::{exc, PyErr, PyObject, PyResult, Python};

use crate::{darken_pixels, extract_blues, ImgError};

py_module_initializer!(img_utils, initimg_utils, PyInit_img_utils, |py, m| {
    m.add(py, "__doc__", "Image manipulation library")?;
    m.add(
        py,
        "darken_pixels",
        py_fn!(
            py,
            darken_pixels_py(src_path: String, dst_path: String, amount: u8, cutoff: u8)
        ),
    )?;
    m.add(
        py,
        "extract_blues",
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
        Err(e) => match e {
            ImgError::FileNotFound => {
                Err(PyErr::new::<exc::RuntimeError, _>(python, "File not found"))
            }
            ImgError::IoError(e) => Err(PyErr::new::<exc::RuntimeError, _>(
                python,
                format!("IO Error: {:?}", e),
            )),
            e => Err(PyErr::new::<exc::RuntimeError, _>(
                python,
                format!("{:?}", e),
            )),
        },
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
        Err(e) => match e {
            ImgError::FileNotFound => {
                Err(PyErr::new::<exc::RuntimeError, _>(python, "File not found"))
            }
            ImgError::IoError(e) => Err(PyErr::new::<exc::RuntimeError, _>(
                python,
                format!("IO Error: {:?}", e),
            )),
            e => Err(PyErr::new::<exc::RuntimeError, _>(
                python,
                format!("{:?}", e),
            )),
        },
    }
}
