use image::imageops::FilterType;
use image::RgbaImage;
use image::DynamicImage;
use std::borrow::Cow;

pub mod boring;

pub trait Comparer {
    type DestImage: Clone;
    /// compare tile to base and output diff
    /// the comparer is responsible for scaling the tile to base's size
    fn compare(base: &Self::DestImage, tile: &Self::DestImage, res: (u32,u32)) -> u64;

    fn pre_parse(i: &DynamicImage, dest: (u32,u32), scale: FilterType) -> Self::DestImage;

    fn pre_parse2(i: &DynamicImage, dest: (u32,u32), scale: FilterType) -> RgbaImage;
}

#[derive(Clone)]
pub enum TileParsable<C> where C: Comparer {
    Raw(DynamicImage),
    Parsed(C::DestImage),
}

impl<C> TileParsable<C> where C: Comparer {
    pub fn parse(&self, dest: (u32,u32), scale: FilterType) -> Cow<C::DestImage> {
        match self {
            Self::Raw(i) => Cow::Owned(C::pre_parse(i,dest,scale)),
            Self::Parsed(i) => Cow::Borrowed(i),
        }
    }
}