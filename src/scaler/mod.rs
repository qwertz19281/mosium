use image::DynamicImage;

pub trait Scaler {
    fn scale(i: &DynamicImage, dest: (usize,usize)) -> DynamicImage;
}
