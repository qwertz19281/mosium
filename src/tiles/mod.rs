pub mod dest_tile;
pub mod src_tile;
pub use dest_tile::*;
pub use src_tile::*;

#[derive(Clone)]
pub struct SrcId {
    pub diff: u64,
    pub id: usize,
}
#[derive(Clone)]
pub struct DestId {
    pub diff: u64,
    pub id: usize,
}

pub type SrcTiles = Vec<SrcTile>;
pub type DestTiles = Vec<DestTile>;