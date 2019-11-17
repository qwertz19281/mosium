use crate::tiles::SrcId;
use image::DynamicImage;
use crate::comparer::Comparer;

pub struct DestTile {
    pub src_matches: Vec<SrcId>,
    pub linked: Option<SrcId>,
}