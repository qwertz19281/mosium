use std::{sync::Arc};
use crate::{comparer::Comparer};
use image::imageops::FilterType;

pub struct Meta<C: Comparer> {
    pub scale: FilterType,
    pub tile_size: (u32,u32),
    pub tile_axis: (u32,u32),
    pub walls_parsed: Vec<C::DestImage>,
}

pub type ArcMeta<C: Comparer> = Arc<Meta<C>>;

#[derive(Clone,Copy)]
pub struct Match {
    pub tile: u32,
    pub wall: u32,
    pub diff: u64,
}