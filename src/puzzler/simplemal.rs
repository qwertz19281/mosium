use super::*;
use crate::tiles::DestId;
use crate::tiles::src_tile::SrcTile;
use crate::tiles::dest_tile::DestTile;
use rand::RngCore;

pub struct SimpleMal;

impl Puzzler for SimpleMal {
    fn puzzle(mut matches: Vec<Match>, wall: &mut [Option<u32>], tiles: usize, noise: usize) {
        for t in matches.iter_mut() {
            if wall[t.wall as usize].is_none() {
                wall[t.wall as usize] = Some(t.tile);
            }
        }
    }
}