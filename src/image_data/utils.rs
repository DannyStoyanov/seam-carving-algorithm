extern crate image;

use image::*;

pub fn print_matrix(matrix: &Vec<Vec<f32>>) {
    for i in 0.. matrix.len() {
        println!("");
        for j in 0.. matrix[i].len() {
            print!("{:?}    ", matrix[i][j]);
        }
    }
}

pub fn max_f32(a: f32, b: f32) -> f32 {
    let eps = 0.0001; // aproximation
    if (a-b) >= eps {
         a
    } 
    else {
         b
    }
}

pub fn min_f32(a: f32, b: f32) -> f32 { 
    let eps = 0.0001; // aproximation
    if (a-b) <= eps {
        a
    } 
    else {
        b
    }
}

pub fn min_of_three(a: f32, b:f32, c:f32) -> f32{
    min_f32(min_f32(a, b), c)
}

pub fn round_f32(numb: f32) -> f32 {
    (numb * 10.0).round() / 10.0
}

pub fn round_rgb_light(val: f32) -> f32 {
    if val < 0.0 {
        0.0
    }
    else if val > 1.0 {
        1.0
    }
    else {
        val
    }
}

pub fn rgb_to_f32_pixel(pixel: &image::Rgba<u8>) -> f32{ // CHECK IF NOT WORKING
    let image::Rgba(data) = *pixel;
    let r: f32 = data[0] as f32 / 255.0;
    let g: f32 = data[1] as f32 / 255.0;
    let b: f32 = data[2] as f32 / 255.0;
    let max_val: f32 = max_f32(r, max_f32(g,b));
    let min_val: f32 = min_f32(r, min_f32(g,b));
    let h: f32 = (max_val + min_val) / 2.0;
    round_f32(h)
}

/// Converting RGB image to light image with f32 values:
pub fn rgb_to_f32_matrix(img: &image::DynamicImage) -> Vec<Vec<f32>> {
    let (imgx, imgy) = img.dimensions();
    let imgx = imgx as usize;
    let imgy = imgy as usize;
    let mut matrix = vec![ vec![0f32; imgx];imgy];
    for x in 0..imgx {
        for y in 0..imgy {
            let pixel = img.get_pixel(x as u32, y as u32);
            let value = rgb_to_f32_pixel(&pixel);
            matrix[y][x] = value;
        }
    }
    matrix
}

/// Converting light image with f32 values to RGB image:
pub fn f32_to_rgb_matrix(matrix: &Vec<Vec<f32>>) -> image::DynamicImage {
    let imgy = matrix.len() as u32;
    let imgx = matrix[0].len() as u32;
    let mut new_img = image::ImageBuffer::new(imgx, imgy);
    
    for y in 0 .. imgy {
        for x in 0 .. imgx {
            let pixel = new_img.get_pixel_mut(x, y);
            let val = (matrix[y as usize][x as usize] * 255 as f32) as u8;
            *pixel = image::Rgb([val, val, val]);
        }
    }
    //new_img.save("data/energy_map.jpg").unwrap();
    DynamicImage::ImageRgb8(new_img)
}

/// Converting light image with f32 values to RGB image:
pub fn u8_to_rgb_matrix(matrix: &Vec<Vec<u8>>) -> image::DynamicImage {
    let imgy = matrix.len() as u32;
    let imgx = matrix[0].len() as u32;
    let mut new_img = image::ImageBuffer::new(imgx, imgy);
    
    for y in 0 .. imgy {
        for x in 0 .. imgx {
            let pixel = new_img.get_pixel_mut(x, y);
            let val = (matrix[y as usize][x as usize] * 255) as u8;
            *pixel = image::Rgb([val, val, val]);
        }
    }
    new_img.save("data/energy_map.jpg").unwrap();
    DynamicImage::ImageRgb8(new_img)
}

/// Finds the minimum pixel value of the three pixels below:
pub fn under_min(matrix: &Vec<Vec<f32>>, row: usize, col:usize) -> (usize, usize, f32) {
    let m = matrix[0].len();
    if col == 0 {
        if matrix[row+1][col] == min_f32(matrix[row+1][col], matrix[row+1][col+1]) {
            (row+1, col, matrix[row+1][col])
        }
        else {
            (row+1, col+1, matrix[row+1][col+1])
        }
    }
    else if col == (m-1) {
        if matrix[row+1][col] == min_f32(matrix[row+1][col-1], matrix[row+1][col]) {
            (row+1, col, matrix[row+1][col])
        }
        else {
            (row+1, col-1, matrix[row+1][col-1])
        }
    }
    else {
        if matrix[row+1][col] == min_of_three(matrix[row+1][col-1], matrix[row+1][col], matrix[row+1][col+1]) {
            (row+1, col, matrix[row+1][col])
        }
        else if matrix[row+1][col-1] == min_of_three(matrix[row+1][col-1], matrix[row+1][col], matrix[row+1][col+1]) {
            (row+1, col-1, matrix[row+1][col-1])
        }
        else {
            (row+1, col+1, matrix[row+1][col+1])
        }
    }
}

/// Creates energy map:
pub fn energy_grid(matrix: &Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut grid: Vec<Vec<f32>> = vec![vec![0.0f32;cols];rows];
    grid[rows-1] = matrix[rows-1].clone();
    let mut row = (rows-2) as i32;
    while row > -1 {
        for col in 0 .. cols {
            let previous_min = under_min(&grid, row as usize, col);
            grid[row as usize][col] = round_f32(matrix[row as usize][col] + previous_min.2);
        }
        row = row - 1;
    }
    grid
} 

/// Finds minimum energy seam from a fixed starting position:
pub fn find_seam_at(matrix: &Vec<Vec<f32>>, start_y: usize) -> Vec<(usize,usize)> {
    let rows = matrix.len();
    let mut path: Vec<(usize,usize)> = Vec::new();
    let start_x = 0;

    path.push((start_x,start_y));
    let mut dir = start_y;
    for row in 0..rows-1 {
        let temp = under_min(matrix, row, dir);
        path.push((temp.0,temp.1));
        dir = temp.1;       
    }

    path
}

