use super::*;
use std::collections::HashSet;

pub struct AllToAll;

impl Mapper for AllToAll {
    fn puzzle(mut matches: Vec<Match>, wall: &mut [Option<u32>], tiles: usize, noise: usize) {
        // All already used tiles to dedup
        let mut taken: HashSet<u32> = HashSet::with_capacity(tiles);
        // iterate through huge and dedup
        loop {
            let mut repeat = false;
            for t in matches.iter_mut() {
                if wall[t.wall as usize].is_none() {
                    if taken.contains(&t.tile) {
                        repeat = true;
                    }else{
                        taken.insert(t.tile);
                        wall[t.wall as usize] = Some(t.tile);
                    }
                }
            }
            if repeat {
                taken.clear();
            }else{
                break;
            }
        }
    }
}
