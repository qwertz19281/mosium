use std::slice::from_raw_parts;
use lab::Lab;
use image::RgbaImage;
use image::DynamicImage;
use crate::comparer::Comparer;
use num_rational::Rational64;
use image::imageops::{FilterType,crop,resize};

pub struct Boring;

impl Comparer for Boring {
    type DestImage = (Box<[Lab]>,Box<[u8]>);

    fn compare(tile: &Self::DestImage, wall: &Self::DestImage, _res: (u32,u32)) -> u64 {
        let mut sum = 0;

        for ((s,sa),(d,da)) in ( tile.0.iter().zip(tile.1.iter()) ) .zip( wall.0.iter().zip(wall.1.iter()) ) {
            //l*a*b* squared distance should be precise enough
            let fdiff = s.squared_distance(d); //max: 30000
            let amul = *(sa.min(da)) as f32;// / 65025.0;
            sum += (fdiff*amul) as u64;
        }

        sum
    }

    fn pre_parse(i: DynamicImage, dest: (u32,u32), scale: FilterType) -> Self::DestImage {
        let i = Self::pre_parse2(i, dest, scale);

        let ac: Vec<u8> = i.pixels().map(|p| p[3] ).collect();

        (image_to_lab(&i),ac.into_boxed_slice())
    }

    fn pre_parse2(i: DynamicImage, dest: (u32,u32), scale: FilterType) -> RgbaImage {
        let mut i = i.to_rgba();
        if (i.width(),i.height()) != dest {
            i = scale_trans(i, dest, scale);
        }
        i
    }
}

fn scale_trans(mut inp: RgbaImage, dest: (u32,u32), scale: FilterType) -> RgbaImage {
    let (iw,ih,ow,oh) = (inp.width() as i64, inp.height() as i64, dest.0 as i64, dest.1 as i64);

    //get the scale factor to determine which side scales more
    let ari = Rational64::new( iw, ih );
    let aro = Rational64::new( ow, oh );

    let (zw,zh,ox,oy);

    if ari >= aro {
        //input is too landscape
        zw = ow * ih / oh;
        zh = ih;
        assert!(zw<=iw);
        ox = (iw-zw)/2;
        oy = 0;
    }else{
        //input is too portrait
        zw = iw;
        zh = oh * iw / ow;
        assert!(zh<=ih);
        ox = 0;
        oy = (ih-zh)/5;
    }

    let i = crop(&mut inp, ox as u32, oy as u32, zw as u32, zh as u32);
    let i = resize(&i, ow as u32, oh as u32, scale);
    i
}

fn image_to_lab(i: &RgbaImage) -> Box<[Lab]>{
    let ptr = (*i).as_ptr();

    let ip = unsafe { from_raw_parts(ptr as *const [u8;3], (*i).len()/3) };

    let labbed = if is_x86_feature_detected!("avx") && is_x86_feature_detected!("sse4.1") {
        lab::simd::rgbs_to_labs(ip)
    } else {
        lab::rgbs_to_labs(ip)
    };

    labbed.into_boxed_slice()
}

#[test]
fn labparse() {
    eprintln!("Lab000000: {:?}",Lab::from_rgb(&[0, 0, 0]));
    eprintln!("LabFFFFFF: {:?}",Lab::from_rgb(&[255, 255, 255]));

    eprintln!("dist1: {:?}",Lab::from_rgb(&[255, 255, 255]).squared_distance(&Lab::from_rgb(&[0, 0, 0])));
    eprintln!("dist1: {:?}",Lab::from_rgb(&[255, 0, 0]).squared_distance(&Lab::from_rgb(&[0, 0, 255])));
}