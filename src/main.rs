use img_utils::darken_pixels;

fn main() {
    darken_pixels(
        String::from("test.jpg"),
        String::from("test-out.jpg"),
        80,
        220,
    )
    .unwrap();
}
