
# truchet-tiles

A simple [Truchet tile](https://en.wikipedia.org/wiki/Truchet_tiles) generator in Rust, makes nice wallpapers and the like.

`$ truchet-tiles <input tile image> <output width> <output height> <output image path>`

Input tile images are mixed with a random color for each tile based on their alpha channel. If the output width/height isn't a multiple of the input tile width/height, it just leaves blank space where it can't fill a whole tile (for now). `tile.png` is an example that can be used for an input tile.

### Example

![Example output](https://github.com/andrew-pa/truchet-tiles/raw/master/out-test.png)



