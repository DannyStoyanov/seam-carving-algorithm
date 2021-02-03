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

/// Creates 3x3 submatrix with 8 neightbours of element with position (x,y):
pub fn submatrix(matrix: &Vec<Vec<f32>>, x: usize, y: usize) -> Vec<Vec<f32>> {
    let mut submatrix: Vec<Vec<f32>> = vec![vec![0.0f32;3];3];
    let mut x1 = 0;
    let mut y1 = 0;
    for i in x-1..x+2 {
        y1 = 0;
        for j in y-1..y+2 {
            submatrix[x1][y1] = matrix[i][j];
            y1 = y1 + 1;
        }
        x1 = x1 + 1;
    }
    submatrix
}

pub fn convolution(matrix: &Vec<Vec<f32>>, filter: &mut Vec<Vec<f32>>) -> f32 {
    filter[0].reverse();
    filter[1].reverse();
    filter[2].reverse();
    filter.reverse();
    //filter.into_iter().map(|arr| arr.reverse()).collect();
    //filter.reverse();

    let result = vec![
         vec![round_f32(matrix[2][2]*filter[0][0]), round_f32(matrix[2][1]*filter[0][1]), round_f32(matrix[2][0]*filter[0][2])] ,
         vec![round_f32(matrix[1][2]*filter[1][0]), round_f32(matrix[1][1]*filter[1][1]), round_f32(matrix[1][0]*filter[1][2])] ,
         vec![round_f32(matrix[0][2]*filter[2][0]), round_f32(matrix[0][1]*filter[2][1]), round_f32(matrix[0][0]*filter[2][2])] 
    ];
    let mut sum = 0.0;
    for arr in result {
        for elem in arr {
            sum = round_f32(sum + elem);
        }
    }
    sum
}

/// Calculates the new value for each pixel:
pub fn sobel_value(matrix: &Vec<Vec<f32>>, x: usize, y: usize) -> f32 {
    let mut gx = vec![
        vec![1.0f32, 0.0, -1.0],
        vec![2.0, 0.0, -2.0],
        vec![1.0, 0.0, -1.0]
        ];
    let mut gy = vec![
        vec![1.0f32, 2.0, 1.0],
        vec![0.0, 0.0, 0.0],
        vec![-1.0, -2.0, -1.0]
        ];
    let matrixA = submatrix(&matrix, x ,y);
    let gXA = convolution(&matrixA, &mut gx);
    let gYA = convolution(&matrixA, &mut gy);

    let sobelVal = round_f32((gXA.powi(2) + gYA.powi(2)).sqrt());
    sobelVal
}

pub fn sobel(submatrix: &Vec<Vec<f32>>, x: usize, y: usize) -> f32 {
    round_rgb_light(sobel_value(&submatrix, x, y))
}

pub fn sobel_edge_detect(img: &image::DynamicImage) -> DynamicImage { // TO BE OPTIMIZED
    let img = img.grayscale();
    let matrix = rgb_to_f32_matrix(&img);
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut new_matrix: Vec<Vec<f32>> = vec![vec![0f32;cols]; rows];
    for i in 1..(rows-1) {
        for j in 1..(cols-1) {
            new_matrix[i][j] = sobel(&matrix, i,j);
        }
    }
    f32_to_rgb_matrix(&new_matrix)
}

pub fn seam_energy(matrix: &Vec<Vec<f32>>, seam: &Vec<(usize,usize)>) -> f32 {
    let mut sum = 0.0;
    for elem in seam {
        sum = sum + matrix[elem.0][elem.1];
    }
    round_f32(sum)
}

pub fn list_of_seams(matrix: &Vec<Vec<f32>>) -> Vec<(Vec<(usize,usize)>, f32)> { // TO BE OPTIMIZED
    let mut seams: Vec<Vec<(usize,usize)>> = Vec::new();
    let mut buffer: Vec<(Vec<(usize,usize)>, f32)> = Vec::new();
    let cols = matrix[0].len();
    for i in 2.. cols-2 { // for energy_map in 2..cols-2, otherwise 0..cols
        let seam = find_seam_at(matrix, i);
        let energy = seam_energy(matrix, &seam);
        seams.push(seam.clone());
        buffer.push((seam, energy)); 
    }
    buffer
}

pub fn find_min_seam(buffer: &mut Vec<(Vec<(usize,usize)>, f32)>) -> Vec<(usize,usize)> {
    let (mut min_seam,mut energy) = buffer[0].clone();
    let mut index=0;
    for i in 1..buffer.len() {
        if energy >= buffer[i].1 {
            min_seam = buffer[i].0.clone();
            energy = buffer[i].1.clone();
            index = i;
        }
    }
    buffer.remove(index).0
}
