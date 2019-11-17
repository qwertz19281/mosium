use std::path::Path;
use std::sync::Arc;
use crate::tiles::DestId;

pub struct SrcTile {
    pub path: Arc<Path>,
    pub dest_matches: Vec<DestId>,
    pub linked: Vec<DestId>,
}

