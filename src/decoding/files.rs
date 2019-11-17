use image::guess_format;
use std::sync::Arc;
use std::path::Path;
use walkdir::WalkDir;
use std::fs::File;
use std::io::Read;

///check if the file is a image by trying to read and guess a sector
pub fn is_image_file(p: &Path) -> bool {
    if let Ok(mut f) = File::open(p) {
        let mut header = vec![0;512];
        if f.read_exact(&mut header).is_ok() {
            if guess_format(&header).is_ok() {
                return true;
            }
        }
    }
    false
}

pub fn collect_files(p: impl AsRef<Path>, recurse: bool, follow_links: bool) -> Vec<Arc<Path>> {
    let mut walker = WalkDir::new(p);
    walker = walker.follow_links(follow_links);
    if !recurse {
        walker = walker.max_depth(0);
    }
    walker.into_iter()
        .filter_map(|e| e.ok() )
        .filter_map(|e| e.metadata().ok().map(|md| (md.is_file(),e) ) )
        .filter(|(ft,e)| {
            *ft && is_image_file(e.path())
        })
        .map(|(_,e)| e.into_path().into() )
        .collect()
}