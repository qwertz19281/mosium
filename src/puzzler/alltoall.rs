use super::*;
use std::collections::HashSet;
use crate::tiles::*;

pub struct AllToAll;

impl Puzzler for AllToAll {
    fn puzzle(mut wall: &mut Vec<DestTile>, tile: &mut Vec<SrcTile>) {
        // free some memory
        /*for dest in wall.iter_mut() {
            dest.src_matches = Vec::new();
        }*/
        // clear the source to dest vec to free some memory
        for src in tile.iter_mut() {
            src.dest_matches = Vec::new();
        }
        // HUGE vector for dest x src
        let mut huge: Vec<(u32,u32,u64)> = Vec::with_capacity(wall.len()*tile.len());
        // fill all matches into the huge vector
        for (di,wall) in wall.iter().enumerate() {
            for src in wall.src_matches.iter() {
                huge.push((src.id as u32,di as u32,src.diff));
            }
        }
        
        // All already used tiles to dedup
        let mut taken: HashSet<u32> = HashSet::with_capacity(tile.len());
        // sort the huge vec by match
        huge.sort_unstable_by_key(|t| t.2 );
        // iterate through huge and dedup
        loop {
            let mut repeat = false;
            for t in huge.iter_mut() {
                if wall[t.1 as usize].linked.is_none() {
                    if taken.contains(&t.0) {
                        repeat = true;
                    }else{
                        taken.insert(t.0);
                        wall[t.1 as usize].linked = Some(SrcId{diff: t.2, id: t.0 as usize});
                    }
                }
            }
            if repeat {
                taken.clear();
            }else{
                break;
            }
        }
        drop(huge);
        // set assocs reverse
        for (i,t) in wall.iter_mut().enumerate() {
            if t.linked.is_none() {
                panic!();
                t.linked = Some(t.src_matches[0].clone());
                //t.linked = Some(SrcId{diff: 0, id: 0});
            }
            let t = t.linked.as_ref().unwrap();
            tile[t.id].linked.push(DestId{diff: t.diff, id: i});
        }
    }
}
