use image::FilterType;
use image::RgbaImage;
use image::DynamicImage;

pub mod boring;

pub trait Comparer {
    type DestImage;
    /// compare tile to base and output diff
    /// the comparer is responsible for scaling the tile to base's size
    fn compare(base: &Self::DestImage, tile: &Self::DestImage, res: (u32,u32)) -> u64;

    fn pre_parse(i: DynamicImage, dest: (u32,u32), scale: FilterType) -> Self::DestImage;

    fn pre_parse2(i: DynamicImage, dest: (u32,u32), scale: FilterType) -> RgbaImage;
}