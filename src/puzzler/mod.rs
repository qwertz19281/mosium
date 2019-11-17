use crate::tiles::src_tile::SrcTile;
use crate::tiles::dest_tile::DestTile;

pub mod simplemal;

pub trait Puzzler {
    fn puzzle(wall: &mut Vec<DestTile>, tile: &mut Vec<SrcTile>);
}