#![windows_subsystem = "windows"] // removes console window while running the exe file
extern crate image;
mod image_data;

use image::*;
use fltk::*; 
use fltk::{app, frame::Frame, image::SharedImage, prelude::*, window::Window,  dialog::*};
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut win = Window::default().with_size(800, 600);
    let mut originalFrame = Frame::default().size_of(&win).with_pos(-200, 0);
    let mut resultFrame = Frame::default().size_of(&win).with_pos(200, 0);
    let mut but = button::Button::new(350,100, 100, 40, "Click");
    let input = input::Input::new(350, 50, 100, 40, "Pixels ot reduce:");

    let mut image = SharedImage::load(Path::new("D:\\Documents\\University\\3rd_course\\Rust\\fltk\\workspace\\data\\sample2.png"))?;
    image.scale(320, 240, true, true);

    originalFrame.set_image(Some(image.clone()));
    resultFrame.set_image(Some(image));

    win.make_resizable(true);
    win.end();
    win.show();

    let (s, r) = app::channel::<bool>();
    but.emit(s, true);

    let mut resultImage: SharedImage;

    while app.wait().unwrap() {
        let seamsNumber = input.value();
        match r.recv() {
            Some(msg) => {
                if msg {
                    let mut img = image_data::ImageData::new(String::from("data/sample2.png")).unwrap();
                    resultFrame.set_image(None::<SharedImage>);
                    win.set_label("Processing!");
                    match img.seam_carving(seamsNumber.parse::<u32>().unwrap()) {
                        Ok(true) => {
                            resultFrame.set_image(None::<SharedImage>);
                            resultFrame.hide();
                            resultImage = SharedImage::load(Path::new("D:\\Documents\\University\\3rd_course\\Rust\\seam_carving\\data\\seam_carving.png"))?;
                            resultImage.scale((320 - seamsNumber.parse::<i32>().unwrap()), 240, true, true); 
                            // resultImage.scale(320, 240, true, true); 
                            resultFrame.set_image(Some(resultImage));
                            resultFrame.show();
                            win.set_label("Ready!");
                        },
                        Ok(false) => (),
                        Err(E) => win.set_label("Error!"),
                    }
                } else {
                }
            }
            None => (),
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    app.run()?;
    Ok(())
}