extern crate image;
use image::{GenericImageView, Pixel};
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

    let samples = [256, 1024, 4096];

    let white_pixel = image::Rgb([255, 255, 255]);

    let sample_image = image::open("images/sample_image.png").unwrap();

    // with noise
    {
        for sample_count in samples.iter() {
            let mut current_image = Image::new(image_size, image_size);
            let mut current_image_sample = Image::new(image_size, image_size);

            for _ in 1..sample_count.clone() {
                let x = die.sample(&mut rng);
                let y = die.sample(&mut rng);
                let current_pixel = current_image.imgbuf.get_pixel_mut(x, y);
                let sample_pixel = current_image_sample.imgbuf.get_pixel_mut(x, y);

                *current_pixel = white_pixel.clone();
                *sample_pixel = Pixel::to_rgb(&sample_image.get_pixel(x, y));
            }
            current_image.save(format!("images/white_noise_{}.png", &sample_count));
            current_image_sample.save(format!("images/white_noise_samples_{}.png", &sample_count));
        }
    }

    // regular samples
    {
        for sample_count in samples.iter() {
            let mut current_image = Image::new(image_size, image_size);
            let mut current_image_sample = Image::new(image_size, image_size);

            let side = (*sample_count as f32).sqrt();
            let pixels = image_size as f32 / side;

            for y in 0..side as i32 {
                let pixel_y = y as f32 * pixels;
                for x in 0..side as i32 {
                    let pixel_x = x as f32 * pixels;

                    let current_pixel = current_image
                        .imgbuf
                        .get_pixel_mut(pixel_x as u32, pixel_y as u32);
                    let sample_pixel = current_image_sample
                        .imgbuf
                        .get_pixel_mut(pixel_x as u32, pixel_y as u32);

                    *current_pixel = white_pixel.clone();
                    *sample_pixel =
                        Pixel::to_rgb(&sample_image.get_pixel(pixel_x as u32, pixel_y as u32));
                }
            }
            current_image.save(format!("images/regular_{}.png", &sample_count));
            current_image_sample.save(format!("images/regular_samples_{}.png", &sample_count));
        }
    }

    // blue noise
    {
        let blue_noise_sample_multiplier = 1;
        for sample_count in samples.iter() {
            let mut current_image = Image::new(image_size, image_size);
            let mut current_image_sample = Image::new(image_size, image_size);

            let mut samples_pos: Vec<(u32, u32)> = Vec::with_capacity(*sample_count);

            for _ in 0..sample_count.clone() {
                // create n points
                let num_candidates = &samples_pos.len() * blue_noise_sample_multiplier + 1;
                let mut best_distance = 0;
                let mut best_candidate_x = 0;
                let mut best_candidate_y = 0;

                for _ in 0..num_candidates {
                    // generate candidates
                    let x = die.sample(&mut rng);
                    let y = die.sample(&mut rng);
                    let mut min_dist: i32 = i32::MAX;

                    for item in &samples_pos {
                        let dist = distance(
                            x as i32,
                            y as i32,
                            item.0 as i32,
                            item.1 as i32,
                            current_image.imgbuf.width() as i32,
                        );
                        if dist < min_dist {
                            min_dist = dist;
                        }
                    }

                    if min_dist > best_distance {
                        best_distance = min_dist;
                        best_candidate_x = x;
                        best_candidate_y = y;
                    }
                }
                samples_pos.push((best_candidate_x, best_candidate_y));
            }

            for pos in samples_pos {
                let x = pos.0;
                let y = pos.1;

                let current_pixel = current_image.imgbuf.get_pixel_mut(x, y);
                let sample_pixel = current_image_sample.imgbuf.get_pixel_mut(x, y);

                *current_pixel = white_pixel.clone();
                *sample_pixel = Pixel::to_rgb(&sample_image.get_pixel(x, y));
            }

            current_image.save(format!("images/blue_noise_{}.png", sample_count));
            current_image_sample.save(format!("images/blue_noise_samples_{}.png", &sample_count));
        }
    }
}
