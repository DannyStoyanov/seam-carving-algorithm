extern crate image;
extern crate rand;

pub mod utils; // must be "pub" if using in main.rs

use image::*;

pub struct ImageData {
    pub path: String,
    image: image::DynamicImage,
    light_matrix: Vec<Vec<f32>>,
    seams: Vec<(Vec<(usize,usize)>, f32)>,
    edge_detect_img: image::DynamicImage, 
}

impl ImageData {
    pub fn new(s: String) -> Result<ImageData, ImageError> {
        match open(s.clone()) {
            Ok(img) => {
                let edges = utils::sobel_edge_detect(&img);
                let val_matrix = utils::rgb_to_f32_matrix(&edges);
                return Ok(ImageData {
                    path: s,
                    image: img.clone(),
                    light_matrix: utils::rgb_to_f32_matrix(&edges),
                    seams: utils::list_of_seams(&val_matrix),
                    edge_detect_img: edges,
                })
            },
            Err(E) => return Err(E),
        }
    }
    
    fn update_data(&mut self, img: image::DynamicImage) {
        let edges = utils::sobel_edge_detect(&img);
        let val_matrix = utils::rgb_to_f32_matrix(&edges);
        self.image = img;
        self.light_matrix =  utils::rgb_to_f32_matrix(&edges);
        //self.seams = utils::list_of_seams(&val_matrix);
        self.edge_detect_img = edges;
    }
    
    pub fn sobel_edge_detect(&self) -> () {
        self.edge_detect_img.save("data/edge_detect.jpg").unwrap();
        ()
    }

    fn remove_seam(&mut self) -> () {
        self.seams = utils::list_of_seams(&self.light_matrix);
        let seam = utils::find_min_seam(&mut self.seams);
    
        let (imgx, imgy) = self.image.dimensions();
        let mut new_img = image::ImageBuffer::new(imgx-1, imgy);
    
        let mut u = 0;
        let mut v = 0;
    
        for y in 0 .. imgy {
            let rgb_to_remove = seam[y as usize];
            v = 0;
            for x in 0 .. imgx {
                if rgb_to_remove != (y as usize, x as usize)    { 
                    let pixel = new_img.get_pixel_mut(v, u);
                    *pixel = self.image.get_pixel(x, y);
                    v = v + 1;
                }  
            }
            u = u + 1;
        }
        //self.image =  DynamicImage::ImageRgba8(new_img);
        self.update_data(DynamicImage::ImageRgba8(new_img));
        ()
    } 

    pub fn seam_carving(&mut self, val: u32){
        for _i in 0..val {
            self.remove_seam();
        }
        self.image.save("data/seam_carving.png").unwrap();
        ()
    }
}