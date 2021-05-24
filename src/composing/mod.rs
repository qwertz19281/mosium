use std::path::Path;
use std::collections::HashMap;
use std::sync::{RwLock, Arc};
use crate::util::transfer;
use image::RgbaImage;
use crate::comparer::Comparer;
use crate::meta::ArcMeta;
use crate::util::RefClonable;
use std::fs::read;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub fn compose_walls<C: Comparer + Send + 'static>(mappings: &[Option<u32>], tile_files: &[Arc<Path>], cscale: u32, meta: ArcMeta<C>) -> RgbaImage where C::DestImage: Send + Sync {
    // collect the wall target per tile
    let mut map: HashMap<Arc<Path>,Vec<usize>> = HashMap::with_capacity(mappings.len());

    for (i,t) in mappings.iter().enumerate() {
        let path = &tile_files[t.unwrap() as usize];
        let entry = map.entry(path.refc());
        entry.or_default().push(i);
    }

    let (tw,tcw,th,tch) = (meta.tile_size.0*cscale, meta.tile_axis.0, meta.tile_size.1*cscale, meta.tile_axis.1);

    let mut out = RgbaImage::new(tw*tcw, th*tch);

    dbg!(tw,tcw,th,tch);

    assert_eq!(tcw*tch, mappings.len() as u32);

    {
        let out = RwLock::new(&mut out);

        map.par_iter().for_each(|(path,dests)| {
            let img = load::<C>(path,cscale,&meta);
            let mut out = out.write().unwrap();

            for i in dests {
                //i is the tile index
                let ox = (*i as u32 % tcw) * tw; 
                let oy = (*i as u32 / tcw) * th; 
                transfer(&mut **out, &img, (tw as i32, th as i32), (0,0), (ox as i32, oy as i32));
            }
        });
    };

    out
}

pub fn load<C: Comparer>(f: &Arc<Path>, cscale: u32, m: &ArcMeta<C>) -> RgbaImage {
    println!("\t{}",f.to_string_lossy());
    let mem = read(&*f).expect("Failed to read Image file");

    let img = image::load_from_memory(&mem[..]).expect("Failed to decode Image file");

    drop(mem);

    let iimg = C::pre_parse2(&img, (m.tile_size.0*cscale,m.tile_size.1*cscale), m.scale);

    iimg
}
