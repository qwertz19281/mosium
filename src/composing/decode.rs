use std::path::Path;
use async_std::fs::read;
use image::RgbaImage;
use crate::meta::ArcMeta;
use std::sync::Arc;
use crate::comparer::Comparer;
use itertools::Itertools;
use async_std::task::block_on;
use async_std::task::spawn;
use crate::util::RefClonable;

pub async fn decode<C: Comparer>(f: Arc<Path>, m: ArcMeta<C>) -> Result<RgbaImage,()> {
    println!("\t{}",f.to_string_lossy());
    let mem = read(&*f).await.expect("Failed to read Image file");

    let img = image::load_from_memory(&mem[..]).expect("Failed to decode Image file");

    drop(mem);

    let iimg = C::pre_parse2(img, m.tile_size, m.scale);

    Ok(iimg)
}
