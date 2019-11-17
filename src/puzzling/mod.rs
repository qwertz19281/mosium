use crate::tiles::SrcId;
use crate::tiles::src_tile::SrcTile;
use crate::tiles::dest_tile::DestTile;
use rayon::prelude::*;

///transfer the diffs into the wall tile vecs
pub fn fill_dest_diffs(s: &mut Vec<DestTile>, r: &Vec<SrcTile>) {
    for dest_tile in s.iter_mut() {
        dest_tile.src_matches.reserve_exact(r.len());
    }

    for (src_id,src_tile) in r.iter().enumerate() {
        for dest_ref in &src_tile.dest_matches {
            s[dest_ref.id].src_matches.push( SrcId{id: src_id, diff: dest_ref.diff} );
        }
    }

    s.par_iter_mut().for_each(|dest_tile| {
        dest_tile.src_matches.sort_unstable_by_key(|a| a.diff );
    });
}