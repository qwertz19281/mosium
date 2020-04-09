use std::mem::swap;
use crate::composing::compose_walls;
use crate::puzzling::fill_dest_diffs;
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
use crate::puzzler::simplemal::SimpleMal;
use crate::puzzler::Puzzler;
use image::ImageFormat;
use puzzler::alltoall::AllToAll;

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
        (version: "0.1")
        (about: "Mosaic Image Generator")
        (@arg NORECURSE: -nr +takes_value "No recursing in input dir")
        (@arg OVERWRITE: -y +takes_value "Allow overwriting output file")
        (@arg TILEW: -j +takes_value "Tile Width (default=64)")
        (@arg TILEH: -k +takes_value "Tile Height (default=64)")
        (@arg INPUT: +required "Input \"Wall\" Image")
        (@arg DIR: +required "Tile Image Dir")
        (@arg OUTPUT: +required "Output Image")
    ).get_matches();

    let recurse = !matches.is_present("NORECURSE");
    let overwrite = !matches.is_present("OVERWRITE");
    
    let tw: u32 = matches.value_of("TILEW").unwrap_or("64").parse().expect("Error: non-number at -tw");
    let th: u32 = matches.value_of("TILEH").unwrap_or("64").parse().expect("Error: non-number at -th");

    let input = matches.value_of("INPUT").expect("Error: Input not set");
    let dir = matches.value_of("DIR").expect("Error: Tile Dir not set");
    let output = matches.value_of("OUTPUT").expect("Error: Output not set");

    let (input, dir, output) = (PathBuf::from(input), PathBuf::from(dir), PathBuf::from(output));

    assert!(input.is_file(),"Error: Input is not an existent file");
    assert!(dir.is_dir(),"Error: Tile Dir is not an existent directory");

    if recurse {
        assert!(!output.is_dir(),"Error: Output is a directory");
    }else{
        assert!(output.exists(),"Error: Output exists");
    }

    run(overwrite, (tw,th), recurse, input, dir, output);
}

pub fn run(overwrite: bool, tile_size: (u32,u32), recurse: bool, input: PathBuf, tile_dir: PathBuf, output: PathBuf) {
    println!("Decode input image");
    //we encode from memory so that image-rs won't guess by extension
    let in_image = read(input).expect("Failed to read input image");
    let in_image = image::load_from_memory(&in_image).expect("Failed to decode input image");
    let in_image = in_image.to_rgba();

    println!("Start finding tile images");
    let tile_files = std::thread::spawn(move || {
        collect_files(tile_dir,recurse,false)
    });

    println!("Split Image in tiles");

    let (walls,walls_parsed,tile_axis) = split_wall::<Boring>(&in_image, false, tile_size, FilterType::Triangle);

    let mut meta = Arc::new(Meta{
        scale: FilterType::Triangle,
        tile_size,
        tile_axis,
        walls_parsed,
        walls,
        src_tiles: Vec::new(),
    });

    print!("Wait for tile search... ");

    let tile_files = tile_files.join().expect("Tile image find thread crashed");

    if tile_files.is_empty() {panic!("No Tile files");}

    println!("DONE");

    println!("Decode and compare tiles");

    let src_tiles = decode_compare_all::<Boring>(tile_files, meta.refc());

    let meta_ref = Arc::get_mut(&mut meta).unwrap();

    meta_ref.src_tiles = src_tiles;

    println!("Extend compare results");

    fill_dest_diffs(&mut meta_ref.walls, &meta_ref.src_tiles);

    println!("Find optimal matches");

    //SimpleMal::puzzle(&mut meta_ref.walls, &mut meta_ref.src_tiles);
    AllToAll::puzzle(&mut meta_ref.walls, &mut meta_ref.src_tiles);

    println!("Compose mosaic image");

    let mut dest_tiles = Vec::new();
    let mut src_tiles = Vec::new();

    swap(&mut dest_tiles, &mut meta_ref.walls);
    swap(&mut src_tiles, &mut meta_ref.src_tiles);

    let composed = compose_walls::<Boring>(dest_tiles, src_tiles, meta.refc());

    println!("Write compose ");

    composed.save_with_format(output, ImageFormat::Png).expect("Failed to write output image");
}