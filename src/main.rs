#![windows_subsystem = "windows"] // removes console window while running the exe file
extern crate image;
mod image_data;

use image::*;
use fltk::*; 
use fltk::{app, frame::Frame, image::SharedImage, prelude::*, window::Window,  dialog::*};
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    // GUI:
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut win = Window::default().with_size(1000, 600);
    let mut originalFrame = Frame::default().size_of(&win).with_pos(-250, 100);
    let mut resultFrame = Frame::default().size_of(&win).with_pos(250, 100);
    let mut but = button::Button::new(450, 125, 100, 40, "Click");
    let mut input = input::Input::new(450, 75, 100, 40, "Pixels to reduce:");

    // Setting frames:
    let mut image = SharedImage::load(Path::new("D:\\Documents\\University\\3rd_course\\Rust\\fltk\\workspace\\data\\sample1.jpg"))?;
    image.scale(400, 275, true, true);
    originalFrame.set_image(Some(image.clone()));
    originalFrame.set_label("Original image");
    resultFrame.set_image(Some(image.clone()));
    resultFrame.set_label("Result image");

    // Button visuals:
    but.clear_visible_focus();
    but.set_color(Color::White);

    // Window visuals:
    win.set_color(Color::DarkCyan);
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
                    let mut img = image_data::ImageData::new(String::from("data/sample1.jpg")).unwrap();
                    resultFrame.set_image(None::<SharedImage>);
                    win.set_label("Processing, please wait!");
                    match img.seam_carving(seamsNumber.parse::<u32>().unwrap()) {
                        Ok(true) => {
                            resultFrame.set_image(None::<SharedImage>);
                            resultFrame.hide();
                            resultImage = SharedImage::load(Path::new("D:\\Documents\\University\\3rd_course\\Rust\\seam_carving\\data\\seam_carving.png"))?;
                            resultImage.scale((400 - seamsNumber.parse::<i32>().unwrap()), 275, false, true); 
                            resultFrame.set_image(Some(resultImage));
                            resultFrame.show();
                            win.set_label("Ready!");
                            //
                            but.hide();
                            input.set_label("Reduced pixels:");
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