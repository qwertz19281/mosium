use crate::meta::ArcMeta;
use std::path::Path;
use std::sync::Arc;
use async_std::future::Future;
use crate::scaler::Scaler;
use crate::tiles::DestId;
use std::error::Error;
use crate::tiles::dest_tile::DestTile;
use crate::tiles::src_tile::SrcTile;
use std::path::PathBuf;
use crate::comparer::Comparer;
use async_std::fs::read;

pub async fn decode_and_compare<C: Comparer>(f: PathBuf, m: ArcMeta<C>) -> Result<SrcTile,Box<dyn Error + Send + Sync>> {
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