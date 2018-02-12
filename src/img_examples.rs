extern crate image;
extern crate num_complex;

use std::fs::File;
use self::image::GenericImage;
use self::num_complex::Complex;

pub fn print_dimensions_and_colortype(img_name: &str) {
	// Use the open function to load an image from a Path.
  // ```open``` returns a `DynamicImage` on success.
  let img = image::open(img_name).unwrap();

  // The dimensions method returns the images width and height.
  println!("dimensions {:?}", img.dimensions());

  // The color method returns the image's `ColorType`.
  println!("{:?}", img.color());

  let ref mut fout = File::create("test.jpeg").unwrap();
  //Write the contents of this image to the Writer in PNG format.
  img.save(fout, image::JPEG).unwrap();
}

pub fn make_julia_fractal(img_name: &str) {
    let max_iterations = 256u16;

    let imgx = 400;
    let imgy = 400;

    let scalex = 4.0 / imgx as f32;
    let scaley = 4.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cy = y as f32 * scaley - 2.0;
        let cx = x as f32 * scalex - 2.0;

        let mut z = Complex::new(cx, cy);
        let c = Complex::new(-0.4, 0.6);

        let mut i = 0;

        for t in 0..max_iterations {
            if z.norm() > 2.0 {
                break
            }
            z = z * z + c;
            i = t;
        }

        // Create an 8bit pixel of type Luma and value i
        // and assign in to the pixel at position (x, y)
        *pixel = image::Luma([i as u8]);

    }

    // Save the image as “fractal.png”
    let ref mut fout = File::create(img_name).unwrap();

    // We must indicate the image's color type and what format to save as
    image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();
}