extern crate image;
use image::{ImageBuffer, RgbImage, Pixel, imageops, FilterType};

pub fn memlyze_image(filepath: &std::path::Path) {
    let mut memlyzer = Memlyzer::from_path(filepath);
    &mut memlyzer.process_image();
    let path_str = &format!("{}.jpeg", filepath.to_str().expect("save error"));
    println!("{}", &path_str);
    let outpath = std::path::Path::new(path_str);
    &memlyzer.save_image(outpath);
}

pub struct Memlyzer {
    in_image: RgbImage,
    out_image: RgbImage,
}

impl Memlyzer {
    fn new(in_image: RgbImage) -> Memlyzer {
        let biggest_of_the_two = std::cmp::max(&in_image.width() * 1, &in_image.height() * 1);
        // because memory limit
        let limit = 150u32;

        if biggest_of_the_two > limit {
            let scale_factor: u32 = (biggest_of_the_two / &limit);
            let in_image = imageops::resize(&in_image, (&in_image.width() / &scale_factor), (&in_image.height() / &scale_factor), FilterType::Lanczos3);
            Memlyzer {
                out_image: ImageBuffer::new(&in_image.width() * &in_image.width(), &in_image.height() * &in_image.height()),
                in_image
            }
        } else {
            Memlyzer {
                out_image: ImageBuffer::new(&in_image.width() * &in_image.width(), &in_image.height() * &in_image.height()),
                in_image
            }
        }
    }
    fn from_path(filepath: &std::path::Path) -> Memlyzer {
        let in_image = image::open(filepath).expect("File Path Error");
        Memlyzer::new(in_image.as_rgb8().expect("Unabled To Convert Image To RGB").clone())
    }
    fn process_image(&mut self) {
        for (c_row, chunk_row) in (0..self.out_image.height()).step_by(self.in_image.height() as usize).enumerate() {
            for (c_col, chunk_col) in (0..self.out_image.width()).step_by(self.in_image.width() as usize).enumerate() {
                let c_pixel = self.in_image.get_pixel(c_col as u32, c_row as u32);
                let c_rgb = c_pixel.to_rgb();
                for row in 0..self.in_image.height() {
                    for col in 0..self.in_image.width() {
                        let pixel = self.in_image.get_pixel(col, row);
                        let mut pixel_rgb = pixel.to_rgb();
                        for (i, v) in c_rgb.data.iter().enumerate() {
//                            println!("v: {} | data: {}", v, &pixel_rgb.data[i]);
                            pixel_rgb.data[i] = ((*v as u32 + pixel_rgb.data[i] as u32) / 2) as u8;
                        }
                        self.out_image.put_pixel(col + chunk_col, row + chunk_row, pixel_rgb);
                    }
                }
            }
        }

    }
    fn save_image(&self, filepath: &std::path::Path) {
        self.out_image.save(filepath);
    }
}