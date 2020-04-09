use crate::composing::compose_walls;
use crate::decoding::decode_compare_all;
use crate::meta::Meta;
use crate::decoding::files::collect_files;
use crate::comparer::boring::Boring;
use crate::decoding::split::split_wall;
use std::fs::read;
use std::sync::Arc;
use std::path::PathBuf;
use image::imageops::FilterType;
use crate::util::RefClonable;
use crate::puzzler::*;
use image::ImageFormat;

#[macro_use]
pub mod util;

pub mod meta;

pub mod decoding;
pub mod tiles;

pub mod opts;
pub mod comparer;
pub mod scaler;

pub mod puzzler;
pub mod puzzling;

pub mod composing;

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(mosion =>
        (version: "0.2")
        (about: "Mosaic Image Generator")
        (@arg NORECURSE: -nr +takes_value "No recursing in input dir")
        (@arg OVERWRITE: -y +takes_value "Allow overwriting output file")
        (@arg TILEW: -j +takes_value "Tile Width (default=64)")
        (@arg TILEH: -k +takes_value "Tile Height (default=64)")
        (@arg CSCALE: -s +takes_value "Compose Scale Factor (default=1)")
        (@arg MAPPER: -m +takes_value "Mapping Algorithm (simple/alltoall) (default=alltoall)")
        (@arg INPUT: +required "Input \"Wall\" Image")
        (@arg DIR: +required "Tile Image Dir")
        (@arg OUTPUT: +required "Output Image")
    ).get_matches();

    let recurse = !matches.is_present("NORECURSE");
    let overwrite = !matches.is_present("OVERWRITE");
    
    let tw: u32 = matches.value_of("TILEW").unwrap_or("64").parse().expect("Error: non-number at -tw");
    let th: u32 = matches.value_of("TILEH").unwrap_or("64").parse().expect("Error: non-number at -th");
    let cscale: u32 = matches.value_of("CSCALE").unwrap_or("1").parse().expect("Error: non-number at -s");

    let input = matches.value_of("INPUT").expect("Error: Input not set");
    let dir = matches.value_of("DIR").expect("Error: Tile Dir not set");
    let output = matches.value_of("OUTPUT").expect("Error: Output not set");

    let (input, dir, output) = (PathBuf::from(input), PathBuf::from(dir), PathBuf::from(output));

    assert!(input.is_file(),"Error: Input is not an existing file");
    assert!(dir.is_dir(),"Error: Tile Dir is not an existing directory");

    if overwrite {
        assert!(!output.is_dir(),"Error: Output is a directory");
    }else{
        assert!(!output.exists(),"Error: Output exists");
    }

    let puzzler = matches.value_of("MAPPER").unwrap_or("alltoall");

    assert!(valid_puzzler(puzzler),"Error: invalid mapper {}",puzzler);

    run((tw,th), cscale, recurse, input, dir, output, puzzler);
}

pub fn run(tile_size: (u32,u32), cscale: u32, recurse: bool, input: PathBuf, tile_dir: PathBuf, output: PathBuf, puzzler: &str) {
    println!("Start finding tile images");
    let tile_files = std::thread::spawn(move || {
        collect_files(tile_dir,recurse,false)
    });

    println!("Decode input image");
    //we encode from memory so that image-rs won't guess by extension
    let in_image_0 = read(input).expect("Failed to read input image");
    let in_image_1 = image::load_from_memory(&in_image_0).expect("Failed to decode input image");
    drop(in_image_0);
    let in_image = in_image_1.to_rgba();
    drop(in_image_1);

    println!("Split Image in tiles");

    let (mut walls_parsed,tile_axis) = split_wall::<Boring>(in_image, false, tile_size, FilterType::Triangle);
    walls_parsed.shrink_to_fit();

    print!("Wait for tile search... ");

    let mut tile_files = tile_files.join().expect("Tile image find thread crashed");
    tile_files.shrink_to_fit();

    println!("DONE");

    if tile_files.is_empty() {panic!("No Tile files");}

    let mut meta = Arc::new(Meta{
        scale: FilterType::Triangle,
        tile_size,
        tile_axis,
        walls_parsed,
    });

    println!("Decode and compare tiles");

    let mut matches = decode_compare_all::<Boring>(tile_files.clone(), meta.refc());

    let meta_ref = Arc::get_mut(&mut meta).unwrap();

    let walls = meta_ref.walls_parsed.len();
    // dealloc parsed walls
    meta_ref.walls_parsed = Vec::new();

    println!("Sort matches");

    matches.sort_unstable_by_key(|t| t.diff );

    println!("Find optimal matches");

    let mut walls_link = vec![None;walls];

    puzzle_with(puzzler, matches, &mut walls_link, tile_files.len(),0).unwrap();

    println!("Compose mosaic image");

    let composed = compose_walls::<Boring>(&walls_link, &tile_files, cscale, meta.refc());

    println!("Write compose");

    composed.save_with_format(output, ImageFormat::Png).expect("Failed to write output image");
}