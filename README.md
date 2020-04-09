# Mosaic Image generator

- WIP & experimental
- implemented in Rust

```
Mosaic Image Generator

USAGE:
    mosion [FLAGS] [OPTIONS] <INPUT> <DIR> <OUTPUT>

FLAGS:
        --no_lab_cache    Disable caching of converted wall. Reduces memory usage, severely hurts performance
    -n                    No recursing in tile dir
    -y                    Allow overwriting output file
    -h, --help            Prints help information
    -V, --version         Prints version information

OPTIONS:
    -p <ACHUNKS>        Analysis concurrency (increases I/O utilization) (default=64)
    -s <CSCALE>         Compose Scale Factor (default=1)
    -m <MAPPER>         Mapping Algorithm (simple/alltoall) (default=alltoall)
    -k <TILEH>          Tile Height (default=64)
    -j <TILEW>          Tile Width (default=64)

ARGS:
    <INPUT>     Input "Wall" Image
    <DIR>       Tile Image Dir
    <OUTPUT>    Output Image
```

## Examples

```
mosion input.jpg tiles mosaic.png # generate mosaic.png using images in tiles/
mosion -s 2 input.jpg tiles mosaic.png # generate 2x size
```

## How does it work?

- Read the input image and split it into tiles
- analyze tile images
  - currently a naive pixel-to-pixel comparision to any tile and calculate the difference
- Use an algorithm to map the tiles using the difference informations
- Generate the image by loading the used tile and putting them into

## Tile Size

The parameters -j and -k define the size of an tile inside the input image (excluding scale factor).  

-s can define a scale factor applied to the generated mosaic.  
Using this is recommended over just upscaling the input image as the analysis can be done in lower resolution and so faster.

## Mapping Algorithms

### Simple

- just maps the best matching tile to any field

### AllToAll

- tries to use as many of the input 
- slightly worse regeneration compared to Simple

## Tile Comparison

is currently implemented by comparing the source tile against all destination tiles pixel-by-pixel in lab colorspace. This is not very performant, but can allow very precise choices in tiles.

## Memory Consumption

Mosion can comsume up to a few gigabytes of memory.

Tips to cut memory usage

- Use -s whenever possible
- As last resort --no_lab_cache can be used. will negatively affecto performance

## TODO

- [x] re-implement noise (to increase randomness of tiles)
- [x] implement any kind of disk caching of comparision data
