use std::path::PathBuf;
use crate::tiles::DestId;

pub struct SrcTile {
    pub path: PathBuf,
    pub dest_matches: Vec<DestId>,
    pub linked: Vec<DestId>,
}

