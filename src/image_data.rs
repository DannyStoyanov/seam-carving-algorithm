extern crate image;
extern crate rand;

pub mod utils; // must be "pub" if using in main.rs

pub struct ImageData {
    pub path: String,
    pub rgb_matrix: image::DynamicImage,
    pub seams: Vec<Vec<f32>>,
}

impl ImageData {

    fn sobel_edge_detect() {
        todo!();
    }

    pub fn seam_carving(){
        todo!();
    }
}