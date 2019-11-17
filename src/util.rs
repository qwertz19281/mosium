use image::RgbaImage;
use std::sync::Arc;

pub trait RefClonable {
    fn refc(&self) -> Self;
}

impl<T> RefClonable for Arc<T> where T: ?Sized {
    #[inline] fn refc(&self) -> Self {
        Arc::clone(self)
    }
}
///transfer a rectange at the given bounds from src to dest
#[inline]
pub fn transfer(dest: &mut RgbaImage, src: &RgbaImage, size: (i32,i32), pos_src: (i32,i32), pos_dest: (i32,i32))  {
    let (w,h) = size;
    let (sox,soy) = pos_src;
    let (dox,doy) = pos_dest;
    let (sw,sh) = (src.width() as i32, src.height() as i32);
    let (dw,dh) = (dest.width() as i32, dest.height() as i32);

    //iter pixel poses in dest cropped to dest size
    for dy in doy.max(0) .. (doy+h).min(dh) {
        for dx in dox.max(0) .. (dox+w).min(dw) {
            //transfer pos to src pos
            let sx = dx-dox+sox;
            let sy = dy-doy+soy;

            //check if src pos in src bounds
            if sx >= 0 && sy >= 0 && sx < sw && sy < sh {
                dest.put_pixel(dx as u32, dy as u32, *src.get_pixel(sx as u32, sy as u32));
            } 
        }
    }
}

/// async parallel mass processing
/// args: input vec, meta arc, #parallel, refname for input, refname for mety, async code block accessing refname'd vars
#[macro_export]
macro_rules! async_par {
    ($inp:ident,$meta:ident,$par:expr,$inp_ref:ident,$meta_ref:ident,$f:block) => {{
        let mut out = Vec::with_capacity($inp.len());

        //so we always spawn up to 64 tasks concurrently and let the stupid executer in parallel
        for c in $inp.into_iter().chunks($par).into_iter() {
            block_on(async {
                let tasks = c
                    .map(|$inp_ref| {
                        let $meta_ref = $meta.refc();
                        spawn(async{ $f })
                    })
                    .collect::<Vec<_>>();

                for t in tasks {
                    if let Ok(r) = t.await {
                        out.push(r);
                    }
                }
            });
        }

        out
    }};
}