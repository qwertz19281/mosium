use image::Rgba;
use std::path::Path;
use std::collections::HashMap;
use std::sync::{RwLock, Arc};
use crate::util::transfer;
use crate::composing::decode::decode_all;
use image::RgbaImage;
use crate::tiles::src_tile::SrcTile;
use crate::tiles::dest_tile::DestTile;
use crate::comparer::Comparer;
use crate::meta::ArcMeta;
use crate::util::RefClonable;
use std::fs::read;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub mod decode;

pub fn compose_walls<C: Comparer + Send + 'static>(inp: Vec<DestTile>, srcs: Vec<SrcTile>, meta: ArcMeta<C>) -> RgbaImage where C::DestImage: Send + Sync {
    //this inefficient hashmap vec thing is still faster than processing pixel masses
    let mut map: HashMap<Arc<Path>,Vec<usize>> = HashMap::with_capacity(inp.len());

    for (i,t) in inp.iter().enumerate() {
        let path = &srcs[t.linked.as_ref().unwrap().id].path;
        let entry = map.entry(path.refc());
        entry.or_default().push(i);
    }

    let (tw,tcw,th,tch) = (meta.tile_size.0, meta.tile_axis.0, meta.tile_size.1, meta.tile_axis.1);

    let mut out = RgbaImage::new(tw*tcw, th*tch);

    dbg!(tw,tcw,th,tch);

    assert_eq!(tcw*tch, inp.len() as u32);

    {
        let out = RwLock::new(&mut out);

        map.par_iter().for_each(|(path,dests)| {
            let img = load::<C>(path,&meta);
            let mut out = out.write().unwrap();

            for i in dests {
                //i is the tile index
                let ox = (*i as u32 % tcw) * tw; 
                let oy = (*i as u32 / tcw) * th; 
                transfer(&mut **out, &img, (tw as i32, th as i32), (0,0), (ox as i32, oy as i32));
            }
        });
    };

    /*for i in 0..(tcw*tch) {
        let ox = (i as u32 % tcw) * tw; 
        let oy = (i as u32 / tcw) * th;

        out.put_pixel(ox,oy, Rgba([(i%255) as u8,0,0,255]));
    }*/

    out
}

pub fn load<C: Comparer>(f: &Arc<Path>, m: &ArcMeta<C>) -> RgbaImage {
    println!("\t{}",f.to_string_lossy());
    let mem = read(&*f).expect("Image failed to read in second pass sowwy");

    let img = image::load_from_memory(&mem[..]).expect("Image suddenly broken is second pass sowwy");

    let iimg = C::pre_parse2(img, m.tile_size, m.scale);

    iimg
}