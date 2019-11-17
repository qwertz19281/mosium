- ### Split Dest Image to Dest Tiles
    - Load the dest image
    - use an alg for optimal insertion into aspect ratio
    - scale/crop
    - divide tiles
- ### Load Src pics
    - for every src image (streamed processing possible)
        - scale them down
        - stretch or crop to tile aspect or use autopan
        - ### Calc Diff between all src vs all dest tiles
            - for pan
            - if autopan
                - inner touch crop, but pan pixel for pixel on src and pick the    one with the highest match
            - else just use the preferred color diff algorithm to compare
        - store
- ### Create Data for preferred puzzler algorithm
    - Every Src Tile
        - Sorted Vec with id of best dest tiles
        - vec of connected dest id's (empty at begin)
    - Every Dest Tile
        - Sorted Vec with id of best src tiles
        - option with connected src id (none at begin)
- ### Run the puzzler algorithm 
- ### Composite the result

Estimated Mem Footprint of puzzle data
#weights = #DestTiles*#SrcTiles*2 + #DestTiles\*2
MiB = weights\*LenInt/1048576