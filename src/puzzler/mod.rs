pub mod simplemal;
pub mod alltoall;

use simplemal::*;
use alltoall::*;
use crate::meta::Match;

pub trait Puzzler {
    fn puzzle(matches: Vec<Match>, wall: &mut [Option<u32>], tiles: usize, noise: usize);
}

pub static PUZZLERS: &'static str = "simple/alltoall";

pub fn puzzle_with(puzzler: &str, matches: Vec<Match>, wall: &mut [Option<u32>], tiles: usize, noise: usize) -> Result<(),()> {
    match &puzzler.to_ascii_lowercase()[..] {
        //puzzle_match!("simple",SimpleMal,"alltoall",AllToAll);
        "simple" => SimpleMal::puzzle(matches,wall,tiles,noise),
        "alltoall" => AllToAll::puzzle(matches,wall,tiles,noise),
        _ => return Err(()),
    }
    Ok(())
}

pub fn valid_puzzler(puzzler: &str) -> bool {
    match &puzzler.to_ascii_lowercase()[..] {
        "simple" => true,
        "alltoall" => true,
        _ => false
    }
}

macro_rules! puzzle_match {
    ($($t:pat,$v:ty),*) => {
        $( $t => $v::puzzle(matches,wall,tiles),
        )*
    };
}