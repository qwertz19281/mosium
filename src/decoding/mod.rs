use std::error::Error;
use crate::tiles::DestId;
use async_std::fs::read;
use crate::meta::ArcMeta;
use async_std::task::block_on;
use async_std::task::spawn;
use crate::tiles::src_tile::SrcTile;
use std::path::PathBuf;
use crate::comparer::Comparer;
use crate::util::RefClonable;
use itertools::Itertools;

pub mod files;
pub mod split;

pub async fn decode_and_compare<C: Comparer>(f: PathBuf, m: ArcMeta<C>) -> Result<SrcTile,Box<dyn Error + Send + Sync>> {
    println!("\t{}",f.to_string_lossy());
    let mem = read(&*f).await?;

    let img = image::load_from_memory(&mem[..])?;

    let iimg = C::pre_parse(img, m.tile_size, m.scale);

    let mut compares: Vec<DestId> = m.walls_parsed.iter().enumerate()
        .map(|(i,t)| DestId{id: i, diff: C::compare(&iimg, t, m.tile_size)} )
        .collect();

    compares.sort_unstable_by_key(|k| k.diff);

    let stile = SrcTile{
        path: f,
        dest_matches: compares,
        linked: Vec::new(),
    };

    

    Ok(stile)
}

pub fn decode_compare_all<C: Comparer + Send + 'static>(p: Vec<PathBuf>, m: ArcMeta<C>) -> Vec<SrcTile> where C::DestImage: Send + Sync {
    async_par!(p,m,64,i,a,{ decode_and_compare::<C>(i,a).await })
}