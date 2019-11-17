use async_std::fs::read;
use image::RgbaImage;
use crate::meta::ArcMeta;
use std::path::PathBuf;
use crate::comparer::Comparer;
use itertools::Itertools;
use async_std::task::block_on;
use async_std::task::spawn;
use crate::util::RefClonable;

pub async fn decode<C: Comparer>(f: PathBuf, m: ArcMeta<C>) -> Result<RgbaImage,()> {
    println!("\t{}",f.to_string_lossy());
    let mem = read(&*f).await.expect("Image failed to read in second pass sowwy");

    let img = image::load_from_memory(&mem[..]).expect("Image suddenly broken is second pass sowwy");

    let iimg = C::pre_parse2(img, m.tile_size, m.scale);

    Ok(iimg)
}

pub fn decode_all<C: Comparer + Send + 'static>(p: Vec<PathBuf>, m: ArcMeta<C>) -> Vec<RgbaImage> where C::DestImage: Send + Sync {
    async_par!(p,m,64,i,a,{ decode::<C>(i,a).await })
}