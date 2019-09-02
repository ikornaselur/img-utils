use crate::errors::ImgError;
use image::{Rgb, RgbImage};

fn _extract_blues(src: RgbImage, min_diff: u8, min_blue: u8) -> Result<RgbImage, ImgError> {
    let mut dst = RgbImage::new(src.width(), src.height());

    dst.pixels_mut()
        .zip(src.pixels())
        .for_each(|(dst_pixel, src_pixel)| {
            let Rgb([r, g, b]) = src_pixel;

            *dst_pixel = if *b >= min_blue
                && ((*b).saturating_sub(*r) >= min_diff || (*b).saturating_sub(*g) >= min_diff)
            {
                Rgb([0, 0, 0])
            } else {
                Rgb([255, 255, 255])
            };
        });

    Ok(dst)
}

pub fn extract_blues(
    src_path: String,
    dst_path: String,
    min_diff: u8,
    min_blue: u8,
) -> Result<(), ImgError> {
    let src = image::open(&src_path)?.to_rgb();

    let dst = _extract_blues(src, min_diff, min_blue)?;
    dst.save(dst_path)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_blues_requires_red_or_green_to_be_min_diff_less_than_blue() {
        #[rustfmt::skip]
        let src: RgbImage = RgbImage::from_vec(9, 1,
            vec![
                50, 50, 50,
                30, 00, 50,
                00, 30, 50,
                30, 30, 50,
                31, 30, 50,
                30, 31, 50,
                80, 80, 50,
                80, 00, 50,
                00, 80, 50,
            ],
        )
        .unwrap();

        let dst = _extract_blues(src, 20, 50).unwrap();

        #[rustfmt::skip]
        assert_eq!(
            dst.into_vec(),
            vec![
                255, 255, 255,
                000, 000, 000,
                000, 000, 000,
                000, 000, 000,
                000, 000, 000,
                000, 000, 000,
                255, 255, 255,
                000, 000, 000,
                000, 000, 000,
            ],
        );
    }
}
