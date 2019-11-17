use crate::tiles::SrcId;
use image::DynamicImage;

pub struct DestTile {
    pub src_matches: Vec<SrcId>,
    pub linked: Option<SrcId>,
    pub val: DynamicImage,
}