pub mod image_data;

use std::cmp;
use image_data::*;
use image::*;
use image::error::*;
// use std::io::Error;
// use std::fmt::Debug;
// use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_f32() {
        assert_eq!(utils::max_f32(0.0000, 0.0001), 0.0001);
        assert_eq!(utils::max_f32(-2.001, 2.001), 2.001);
        assert_eq!(utils::max_f32(-2.001, -2.000), -2.0);
        assert_eq!(utils::max_f32(-2.000001, -2.000001), -2.000001);
        assert_eq!(utils::max_f32(1.2345678, 1.2345789), 1.2345789);
    }

    #[test]
    fn test_min_f32() {
        assert_eq!(utils::min_f32(0.0000, 0.0001), 0.0);
        assert_eq!(utils::min_f32(-2.001, 2.001), -2.001);
        assert_eq!(utils::min_f32(-2.001, -2.000), -2.001);
        assert_eq!(utils::min_f32(-2.000001, -2.000001), -2.000001);
        assert_eq!(utils::min_f32(1.2345678, 1.2345789), 1.2345678);
    }

    #[test]
    fn test_min_of_three() {
        assert_eq!(utils::min_of_three(0.0000, 0.0001, 0.0002), 0.0);
        assert_eq!(utils::min_of_three(0.0001, 0.0000, 0.0002), 0.0);
        assert_eq!(utils::min_of_three(0.0001, 0.0002, 0.0000), 0.0);
        assert_eq!(utils::min_of_three(-0.0001, -0.0002, -0.0003), -0.0003);
        assert_eq!(utils::min_of_three(0.0001, -0.0002, 0.0003), -0.0002);
        assert_eq!(utils::min_of_three(-0.0001, -0.0002, 0.0003), -0.0002);
    }

    #[test]
    fn test_round_f32() {
        assert_eq!(utils::round_f32(0.0000), 0.0);
        assert_eq!(utils::round_f32(0.01), 0.0);
        assert_eq!(utils::round_f32(0.02), 0.0);
        assert_eq!(utils::round_f32(0.03), 0.0);
        assert_eq!(utils::round_f32(0.04), 0.0);
        assert_eq!(utils::round_f32(0.05), 0.1);
        assert_eq!(utils::round_f32(0.06), 0.1);
        assert_eq!(utils::round_f32(0.07), 0.1);
        assert_eq!(utils::round_f32(0.08), 0.1);
        assert_eq!(utils::round_f32(0.09), 0.1);
    }

    #[test]
    fn test_round_rgb_light() {
        assert_eq!(utils::round_rgb_light(0.5), 0.5);
        assert_eq!(utils::round_rgb_light(-0.5), 0.0);
        assert_eq!(utils::round_rgb_light(5.0), 1.0);
    }

    #[test]
    fn test_submatrix() {
        let matrix = vec![vec![1.0_f32, 5.0_f32, 9.0_f32, 2.0_f32], 
                          vec![0.0_f32, 4.0_f32, 6.0_f32, 1.0_f32],
                          vec![6.0_f32, 1.0_f32, 8.0_f32, 8.0_f32], 
                          vec![4.0_f32, 7.0_f32, 3.0_f32, 5.0_f32]];
        
        
        let result = vec![vec![1.0_f32, 5.0_f32, 9.0_f32], 
                          vec![0.0_f32, 4.0_f32, 6.0_f32], 
                          vec![6.0_f32, 1.0_f32, 8.0_f32]];
        assert_eq!(utils::submatrix(&matrix, 1, 1), result); 
    }
    #[test]
    #[should_panic]
    fn test_submatrix_panic() {
        let matrix = vec![vec![1.0_f32, 5.0_f32, 9.0_f32, 2.0_f32], 
                          vec![0.0_f32, 4.0_f32, 6.0_f32, 1.0_f32],
                          vec![6.0_f32, 1.0_f32, 8.0_f32, 8.0_f32], 
                          vec![4.0_f32, 7.0_f32, 3.0_f32, 5.0_f32]];
        
        
        let result = vec![vec![1.0_f32, 5.0_f32, 9.0_f32], 
                          vec![0.0_f32, 4.0_f32, 6.0_f32], 
                          vec![6.0_f32, 1.0_f32, 8.0_f32]];
        assert_eq!(utils::submatrix(&matrix, 0, 0), result); 
        assert_eq!(utils::submatrix(&matrix, matrix.len(), matrix[0].len()), result);
    }

    #[test] 
    #[should_panic]
    fn test_image_data_new() {
        let img1 = image_data::ImageData::new(String::from("invalid_path/image.png")).unwrap();
        let img2 = image_data::ImageData::new(String::from("data/test_img_2.png"));
        match img2 {
            Ok(img) => (),
            Err(E) => (),
        }    
    }
}