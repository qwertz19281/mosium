use crate::tiles::SrcId;
use crate::tiles::src_tile::SrcTile;
use crate::tiles::dest_tile::DestTile;
use rayon::prelude::*;

///transfer the diffs into the wall tile vecs
pub fn fill_dest_diffs(s: &mut Vec<DestTile>, r: &Vec<SrcTile>) {
    //for every dest tile
    s.par_iter_mut().enumerate().for_each(|(our_id,dest_tile)| {
        dest_tile.src_matches.reserve_exact(r.len());
        //iterate over src tiles
        for (their_id,src_tile) in r.iter().enumerate() {
            //find the diff against us
            let id = src_tile.dest_matches.iter()
                .find(|a| a.id == our_id )
                .unwrap();

            dest_tile.src_matches.push( SrcId {id: their_id, diff: id.diff} );
        }
        dest_tile.src_matches.sort_unstable_by_key(|a| a.diff );
    });
}