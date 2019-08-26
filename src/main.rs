use img_utils::increase_contrast;


fn main() {
    increase_contrast(String::from("test.jpg"), String::from("test-out.jpg"), 80, 220).unwrap();
}
