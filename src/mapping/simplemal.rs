use super::*;
use rand::RngCore;

pub struct SimpleMal;

impl Mapper for SimpleMal {
    fn puzzle(mut matches: Vec<Match>, wall: &mut [Option<u32>], tiles: usize, noise: usize) {
        for t in matches.iter_mut() {
            if wall[t.wall as usize].is_none() {
                wall[t.wall as usize] = Some(t.tile);
            }
        }
    }
}