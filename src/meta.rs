use crate::tiles::src_tile::SrcTile;
use crate::tiles::dest_tile::DestTile;
use std::sync::Arc;
use crate::comparer::Comparer;
use image::imageops::FilterType;

pub struct Meta<C: Comparer> {
    pub scale: FilterType,
    pub tile_size: (u32,u32),
    pub tile_axis: (u32,u32),
    pub walls_parsed: Vec<C::DestImage>,
    pub walls: Vec<DestTile>,
    pub src_tiles: Vec<SrcTile>,
}

pub type ArcMeta<C: Comparer> = Arc<Meta<C>>;