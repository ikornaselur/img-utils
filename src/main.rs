use img_utils::increase_contrast;


fn main() {
    increase_contrast(String::from("./example.jpg"), String::from("out.jpg"), 80, 220).unwrap();
}
