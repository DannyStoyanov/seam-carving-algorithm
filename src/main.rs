extern crate image;

mod image_data;

use image::*;

fn main() {
    let mut img = image_data::ImageData::new(String::from("data/sample2.png")).unwrap();
    img.seam_carving(100);
}
