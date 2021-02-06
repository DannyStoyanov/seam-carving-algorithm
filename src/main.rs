extern crate image;

mod image_data;

use image::*;

fn main() {
    let mut img = image_data::ImageData::new(String::from("data/sample1.jpg")).unwrap();
    img.sobel_edge_detect();
    img.seam_carving(66);
}
