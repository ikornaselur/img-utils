use img_utils::{darken_pixels, ImgError};

fn main() -> Result<(), ImgError> {
    darken_pixels(
        String::from("test-img/test-normal.jpg"),
        String::from("/tmp/test-out.jpg"),
        80,
        220,
    )?;

    Ok(())
}
