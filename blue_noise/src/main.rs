extern crate image;
use rand::distributions::{Distribution, Uniform};

fn distance(x_1: i32, y_1: i32, x_2: i32, y_2: i32, image_width: i32) -> i32 {
    let mut d_x = (x_2 - x_1).abs();
    let mut d_y = (y_2 - y_1).abs();

    if d_x > (image_width / 2) {
        d_x = image_width - d_x;
    }
    if d_y > (image_width / 2) {
        d_y = image_width - d_y;
    }

    return d_x * d_x + d_y * d_y;
}

struct Image {
    imgbuf: image::RgbImage,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let mut imgbuf = image::ImageBuffer::new(width, height);

        // Fill all pixels with black
        for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
        }

        Self { imgbuf }
    }

    pub fn save(self, name: String) {
        self.imgbuf.save(name).unwrap();
    }
}

fn main() {
    let image_size = 256;

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..image_size - 1);

    let samples = [];

    let white_pixel = image::Rgb([255, 255, 255]);

    // with noise
    {
        for sample_count in samples.iter() {
            let mut current_image = Image::new(image_size, image_size);

            for _ in 1..sample_count.clone() {
                let x = die.sample(&mut rng);
                let y = die.sample(&mut rng);
                let current_pixel = current_image.imgbuf.get_pixel_mut(x, y);

                *current_pixel = white_pixel.clone()
            }
            current_image.save(format!("with_noise_samples_{}.png", &sample_count));
        }
    }

}
