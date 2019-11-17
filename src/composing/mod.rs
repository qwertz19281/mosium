use crate::util::transfer;
use crate::composing::decode::decode_all;
use image::RgbaImage;
use crate::tiles::src_tile::SrcTile;
use crate::tiles::dest_tile::DestTile;
use crate::comparer::Comparer;
use crate::meta::ArcMeta;
use crate::util::RefClonable;

pub mod decode;

pub fn compose_walls<C: Comparer + Send + 'static>(inp: Vec<DestTile>, srcs: Vec<SrcTile>, meta: ArcMeta<C>) -> RgbaImage where C::DestImage: Send + Sync {
    let src_paths = inp.into_iter()
        .map(|d| srcs[d.linked.unwrap().id].path.clone() )
        .collect::<Vec<_>>();
    
    let tiles = decode_all(src_paths, meta.refc());

    let (tw,tcw,th,tch) = (meta.tile_size.0, meta.tile_axis.0, meta.tile_size.1, meta.tile_axis.1);

    let mut out = RgbaImage::new(tw*tcw, th*tch);

    dbg!(tw,tcw,th,tch);

    assert_eq!(tcw*tch, tiles.len() as u32);

    for (i,t) in tiles.iter().enumerate() {
        let ox = (i as u32 % tcw) * tw; 
        let oy = (i as u32 / tch) * th; 
        transfer(&mut out, t, (tw as i32, th as i32 ), (0,0), (ox as i32, oy as i32));
    }

    out
}