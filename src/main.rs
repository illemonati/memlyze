extern crate memlyze_image;

fn main() {
    memlyze_image::memlyze_image(std::path::Path::new("image/speedwagon.jpeg"));
}