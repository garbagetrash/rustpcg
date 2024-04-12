use image;

use image::{ImageBuffer, Rgb};

pub fn render_greyscale(filename: &str, map: &[Vec<u8>]) {
    // a default (black) image containing Rgb values
    let width = map.len() as u32;
    let height = map[0].len() as u32;
    let mut image = ImageBuffer::new(width, height);

    // set a central pixel to white
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let v = map[x as usize][y as usize];
        *pixel = Rgb([v, v, v]);
    }

    // write it out to a file
    image.save(filename).expect("failed to save output image");
}
