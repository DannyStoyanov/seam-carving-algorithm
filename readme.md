
# seam-carving

## Introduction

Seam-carving is an algorithm for content-aware image resizing. It allows image to be resized without losing important content from scaling or cropping.


## Requirements

- Rust - language
- Cargo - packet manager
- crates/packages:
    - image
    - fltk
    - rand


## Usage

    cargo run

The program is run via the command-line. You have to be in the project's directory and run it via "cargo run" command. Once the project is build and executed you will see a window with simple ui.


## Example Results

The input image is on the left and the output image is on the right.\
![Input image 1](/data/example_1.png) ![Output image 1](/data/seam_carving_example_1.png)

![Input image 2](/data/example_2.jpg) ![Output image 2](/data/seam_carving_example_2.png)
<br/> 

GUI input screenshot:
![GUI example input](/data/before.png)

 <br/>

GUI output screenshot:
![GUI example output](/data/after.png)
<br/>

## Acknowledgements and Links

Acknowledgements to:
- https://github.com/AndrewRadev
- https://github.com/andrewdcampbell/seam-carving
- https://github.com/MoAlyousef

For more information you can check out these links:
- https://en.wikipedia.org/wiki/Seam_carving
- http://cs.brown.edu/courses/cs129/results/proj3/taox/
- https://computationalthinking.mit.edu/Fall20/lecture4/
