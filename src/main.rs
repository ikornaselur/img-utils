use img_utils::{darken_pixels, ImgError};

fn main() -> Result<(), ImgError> {
    darken_pixels(
        String::from("tests/assets/test-small.jpg"),
        String::from("/tmp/test-out.jpg"),
        80,
        220,
    )?;

    Ok(())
}
