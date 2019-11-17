use image::FilterType;
use crate::comparer::Comparer;
use crate::tiles::dest_tile::DestTile;
use crate::util::transfer;
use image::RgbaImage;
use num_rational::Ratio;
use image::DynamicImage;

pub fn split_wall<C: Comparer>(wall: &RgbaImage, crop: bool, (tw,th): (u32,u32), scale: FilterType) -> (Vec<DestTile>,Vec<C::DestImage>,(u32,u32)) {
    //extended image size
    let ww = *Ratio::new( wall.width(), tw ).ceil().numer() * tw;
    let wh = *Ratio::new( wall.height(), th ).ceil().numer() * th;

    //offsets
    let ox = ((ww - wall.width())/2) as i32;
    let oy = ((wh - wall.height())/2) as i32;

    //count of tiles in x and y
    let tcw = ww/tw;
    let tch = wh/th;

    dbg!(tw,tcw,ww,th,tch,wh,ox,oy);

    let mut tiledata = Vec::with_capacity((tcw*tch) as usize);
    let mut tiles = Vec::with_capacity((tcw*tch) as usize);

    for y in 0..tch {
        for x in 0..tcw {
            //absolute offsets plz
            let ax = (x*tw) as i32;
            let ay = (y*th) as i32;

            let mut tile = RgbaImage::new(tw,th);
            transfer(&mut tile, wall, (tw as i32, th as i32), (ax-ox, ay-oy), (0, 0));

            let labbed = C::pre_parse(DynamicImage::ImageRgba8(tile), (tw,th), scale);
            tiles.push(labbed);

            let td = DestTile{
                src_matches: Vec::new(),
                linked: None,
            };
            tiledata.push(td);
        }
    }

    assert_eq!((tcw*tch) as usize, tiledata.len());
    assert_eq!((tcw*tch) as usize, tiles.len());

    (tiledata,tiles,(tcw,tch))
}