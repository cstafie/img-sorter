mod img_examples;
mod lib;

extern crate image;

use std::fs::File;
use image::{Rgb, Pixel, GenericImage, ImageBuffer};
use std::cmp;
use std::vec::Vec;


fn main() {
	let img_name = "pride.jpg"; // 300 x 300 image
	let chunk_size = 10; // TODO: no hard coded values

	let img = image::open(img_name).unwrap();
	let (width, height) = img.dimensions();
	println!("dimensions {:?}", (width, height));

	let mut pixel_vector = Vec::new();

	for x in 0..width {
		for y in 0..height {
			pixel_vector.push(img.get_pixel(x,y).to_rgb());
		}
	}

	pixel_vector.sort_by(|a,b| {
		let (ha, sa, va) = rgb_to_hsv(a);
		let (hb, sb, vb) = rgb_to_hsv(b);

		//(sa, va, ha).partial_cmp(&(sb, vb, hb)).unwrap()
		//(va, sa, ha).partial_cmp(&(vb, sb, hb)).unwrap()
		//(sa, ha, va).partial_cmp(&(sb, hb, vb)).unwrap()
		//(ha, va, sa).partial_cmp(&(hb, vb, sb)).unwrap()
		//(va, ha, sa).partial_cmp(&(vb, hb, sb)).unwrap()
		rgb_to_hsv(a).partial_cmp(&rgb_to_hsv(b)).unwrap()
	});

	let mut sorted_image = image::ImageBuffer::new(width, height);

	for (index, pixel) in pixel_vector.into_iter().enumerate() {
		sorted_image.put_pixel(
			index as u32 % width, 
			index as u32 / width, 
			pixel,
		);
	}	

	//let ref mut fout = File::create("the_shit.png").unwrap();
	sorted_image.save("my_file.png").unwrap();
	//sorted_image.save(fout).unwrap();
}





fn min_float(a: f64, b: f64) -> f64 {
	match a.partial_cmp(&b).unwrap() {
		std::cmp::Ordering::Greater => b,
	  _ => a,
	}
}

fn min_triple(a: f64, b: f64, c: f64) -> f64 {
	min_float(min_float(a, b), c)
}

fn max_float(a: f64, b: f64) -> f64 {
	match a.partial_cmp(&b).unwrap() {
		std::cmp::Ordering::Less => b,
	  _ => a,
	}
}

fn max_triple(a: f64, b: f64, c: f64) -> f64 {
	max_float(max_float(a, b), c)
}




fn rgb_to_hsv(rgb_pixel: &image::Rgb<u8>) -> (f64, f64, f64) {
	let (r,g,b) = (
		rgb_pixel.data[0] as f64 / 255f64,
		rgb_pixel.data[1] as f64 / 255f64, 
		rgb_pixel.data[2] as f64 / 255f64,
	);
	let mut h:f64;
	let mut s = 0f64;
	let v:f64;

	let min = min_triple(r,g,b);
	let max = max_triple(r,g,b);
	v = max;
	let delta = max - min;

	if max != 0f64 {
		s  = delta / max;
	} else {
		h = -1f64;
		return (h,s,v);
	}

	if delta == 0f64 {
		h = 0f64;
	} else if r == max {
		h = (g - b) / delta;
	} else if g == max {
		h = 2f64 + (b - r) / delta;
	} else {
		h = 4f64 + (r - g) / delta;
	}

	h  *= 60f64;

	if h < 0f64 {
		h += 360f64;
	}

	(h,s,v)
}

#[allow(dead_code)]
fn run_image_examples() {
	img_examples::print_dimensions_and_colortype("test.png");
	img_examples::make_julia_fractal("julia_fract.png");
}
