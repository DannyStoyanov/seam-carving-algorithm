extern crate image;
extern crate rand;

pub mod utils;

use image::*;
use image::error::*;
use std::io::{Error, ErrorKind};

pub struct ImageData {
    pub path: String,
    pub rgb_image: image::DynamicImage,
    f32_image: Vec<Vec<f32>>,
    rgb_edge_detect: image::DynamicImage,
    f32_edge_detect: Vec<Vec<f32>>,
    //seams: Vec<(Vec<(usize,usize)>, f32)>
}

impl ImageData {

    pub fn new(s: String) -> Result<ImageData, ImageError> {
        match open(s.clone()) {
            Ok(img) => {
                let (width, height) = img.dimensions();
                if width <= 3 {
                    return Err(ImageError::Limits(LimitError::from_kind(LimitErrorKind::DimensionError)));
                }
                let f32_matrix = utils::rgb_to_f32_matrix(&img);
                let rgb_edge_matrix = utils::sobel_edge_detect(&img);
                let f32_edge_matrix = utils::rgb_to_f32_matrix(&rgb_edge_matrix);
                return Ok(ImageData {
                    path: s,
                    rgb_image: img,
                    f32_image: f32_matrix,
                    rgb_edge_detect: rgb_edge_matrix,
                    f32_edge_detect: f32_edge_matrix.clone(),
                    //seams: utils_v2::list_of_seams(&f32_edge_matrix),
                })
            },
            Err(E) => return Err(E),
        }
    }

    fn save_color_image(&self) {
        self.rgb_image.save("data/seam_carving.png").unwrap();
    }

    fn save_energy_image(&self) {
        self.rgb_edge_detect.save("data/edge_detect.png").unwrap();
    }

    fn edge_detect(&self) -> Vec<Vec<f32>> {
        // let img = self.rgb_image.clone();
        // let img = img.grayscale();
        // let matrix = utils_v2::rgb_to_f32_matrix(&img);
        let matrix = self.f32_image.clone();
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut new_matrix: Vec<Vec<f32>> = vec![vec![0f32;cols]; rows];
        for i in 1..(rows-1) {
            for j in 1..(cols-1) {
                new_matrix[i][j] = utils::sobel(&matrix, i,j);
            }
        }
        //utils_v2::f32_to_rgb_matrix(&new_matrix)
        new_matrix
    }

    fn update_data(&mut self, new_image: image::DynamicImage) {
        self.rgb_image = new_image.clone();
        self.f32_image = utils::rgb_to_f32_matrix(&new_image);
        self.f32_edge_detect = self.edge_detect();
        self.rgb_edge_detect = utils::f32_to_rgb_matrix(&self.edge_detect());
        //self.f32_edge_detect = utils::rgb_to_f32_matrix(&self.edge_detect().clone());
    }

    fn find_min_seam2(&mut self) -> Vec<(usize,usize)> {
        let mut seams = utils::list_of_seams(&self.f32_edge_detect);
        let (mut min_seam, mut energy) = seams[0].clone();
        let mut index=0;
        for i in 1..seams.len() {
            if energy >= seams[i].1 {
                min_seam = seams[i].0.clone();
                energy = seams[i].1.clone();
                index = i;
            }
        }
        seams.remove(index).0
    }

    fn remove_seam(&mut self) {
        let seam = self.find_min_seam2();
    
        let (imgx, imgy) = self.rgb_image.dimensions();
        let mut new_img = image::ImageBuffer::new(imgx-1, imgy);
    
        let mut u = 0;
        let mut v = 0;

        for y in 0 .. imgy {
            let rgb_to_remove = seam[y as usize];
            v = 0;
            for x in 0 .. imgx {
                if rgb_to_remove != (y as usize, x as usize)    { 
                    let pixel = new_img.get_pixel_mut(v, u);
                    *pixel = self.rgb_image.get_pixel(x, y);
                    v = v + 1;
                }  
            }
            u = u + 1;
        }
        self.update_data(DynamicImage::ImageRgba8(new_img));
    }
    
    pub fn seam_carving(&mut self, val: u32) -> Result<bool, ErrorKind> { // VALIDATE INPUT
        if (val < 1) || (val > self.rgb_image.width()) {
            return Err(ErrorKind::InvalidInput);
        }
        for _i in 0..val {
            self.remove_seam();
        }
        self.save_energy_image();
        self.save_color_image();
        Ok(true)
    }
}