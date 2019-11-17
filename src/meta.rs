use crate::tiles::SrcTiles;
use crate::tiles::DestTiles;
use std::sync::Arc;
use crate::comparer::Comparer;
use image::FilterType;

pub struct Meta<C: Comparer> {
    pub scale: FilterType,
    pub tile_size: (u32,u32),
    pub tile_axis: (u32,u32),
    pub walls_parsed: Vec<C::DestImage>,
    pub walls: Vec<DestTiles>,
    pub src_tiles: Vec<SrcTiles>,
}

pub type ArcMeta<C: Comparer> = Arc<Meta<C>>;