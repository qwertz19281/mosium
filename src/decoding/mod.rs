use std::error::Error;
use async_std::fs::read;
use crate::meta::ArcMeta;
use async_std::task::block_on;
use async_std::task::spawn;
use std::sync::Arc;
use std::path::Path;
use crate::comparer::Comparer;
use crate::{meta::Match, util::RefClonable};
use itertools::Itertools;

pub mod files;
pub mod split;

pub async fn decode_and_compare<C: Comparer>(f: Arc<Path>, m: ArcMeta<C>, tile_id: u32) -> Result<Vec<Match>,Box<dyn Error + Send + Sync>> {
    println!("\t{}",f.to_string_lossy());
    let mem = read(&*f).await?;

    let img = image::load_from_memory(&mem[..])?;

    drop(mem);

    let iimg = C::pre_parse(&img, m.tile_size, m.scale);

    drop(img);

    let mut dest = Vec::with_capacity(m.walls_parsed.len());

    for (i,w) in m.walls_parsed.iter().enumerate() {
        let diff = C::compare(&iimg, w.parse(m.tile_size,m.scale).as_ref(), m.tile_size);
        dest.push(Match{wall: i as u32, tile: tile_id, diff});
    }

    Ok(dest)
}

pub fn decode_compare_all<C: Comparer + Send + 'static>(p: Vec<Arc<Path>>, m: ArcMeta<C>) -> Vec<Match> where C::DestImage: Send + Sync {
    async_par!(p,m,m.achunks,f,a,i,{ decode_and_compare::<C>(f,a,i as u32).await })
}
