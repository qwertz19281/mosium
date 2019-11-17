use crate::tiles::DestId;
use crate::tiles::src_tile::SrcTile;
use crate::tiles::dest_tile::DestTile;
use crate::puzzler::Puzzler;

pub struct SimpleMal;

impl Puzzler for SimpleMal {
    fn puzzle(wall: &mut Vec<DestTile>, tile: &mut Vec<SrcTile>) {
        for (i,dest_tile) in wall.iter_mut().enumerate() {
            let matching = &dest_tile.src_matches[0];

            dest_tile.linked = Some(matching.clone());

            tile[matching.id].linked.push(DestId{id: i, diff: matching.diff});
        }
    }
}