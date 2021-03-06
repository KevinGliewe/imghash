# imghash

Creates perceptual hash values from images.

This is a CLI interface for the [img_hash](https://github.com/abonander/img_hash) library.


```
USAGE:
    imghash.exe [OPTIONS] <IMAGE> [IMAGE_CMP]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --hash_alg <hash_alg>              Hash algorithm: Default = Gradient [possible values: Mean, Gradient,
                                           VertGradient, DoubleGradient, Blockhash]
    -h, --height <height>                  Hash height: Default = 8
    -f, --resize_filter <resize_filter>    Resize Filter: Default = Lanczos3 [possible values: Nearest, Triangle,
                                           CatmullRom, Gaussian, Lanczos3]
    -w, --width <width>                    Hash width: Default = 8

ARGS:
    <IMAGE>        Sets the input image path
    <IMAGE_CMP>    Sets the compare image path (If not set, the hash of IMAGE will get printet out)
```

## Options

### Width

`-w` / `--width`

Hash width in bit.

### Height

`-h` / `--height`

Hash height in bit.

### Hash algorithm

`-a` / `--hash_alg`

Each algorithm has different performance characteristics.

https://docs.rs/img_hash/3.2.0/img_hash/enum.HashAlg.html

 * `Mean`:

    The image is converted to grayscale, scaled down to hash_width x hash_height, the mean pixel value is taken, and then the hash bits are generated by comparing the pixels of the descaled image to the mean.

    This is the most basic hash algorithm supported, resistant only to changes in resolution, aspect ratio, and overall brightness.

    Further Reading: http://www.hackerfactor.com/blog/?/archives/432-Looks-Like-It.html

 * `Gradient`:

    The image is converted to grayscale, scaled down to (hash_width + 1) x hash_height, and then in row-major order the pixels are compared with each other, setting bits in the hash for each comparison. The extra pixel is needed to have hash_width comparisons per row.

    This hash algorithm is as fast or faster than Mean (because it only traverses the hash data once) and is more resistant to changes than Mean.

    Further Reading: http://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html

 * `VertGradient`:
    
    Equivalent to Gradient but operating on the columns of the image instead of the rows.

 * `DoubleGradient`:

    An advanced version of Gradient; resizes the grayscaled image to (width / 2 + 1) x (height / 2 + 1) and compares columns in addition to rows.

    This algorithm is slightly slower than Gradient (resizing the image dwarfs the hash time in most cases) but the extra comparison direction may improve results (though you might want to consider increasing hash_size to accommodate the extra comparisons).

    Compared to the other algorithms, this does not require any preprocessing steps and so may be significantly faster at the cost of some resilience.

    The algorithm is described in a high level here: https://github.com/commonsmachinery/blockhash-rfc/blob/master/main.md

 * `Blockhash`:

### Resize filter

`-f` / `--resize_filter`

https://docs.rs/img_hash/3.2.0/img_hash/enum.FilterType.html

 * `Nearest`:

    ![Nearest Neighbor](https://raw.githubusercontent.com/image-rs/image/master/examples/scaledown/scaledown-test-near.png)

 * `Triangle`:

    ![Linear: Triangle](https://raw.githubusercontent.com/image-rs/image/master/examples/scaledown/scaledown-test-tri.png)
 
 * `CatmullRom`:

    ![Cubic: Catmull-Rom](https://raw.githubusercontent.com/image-rs/image/master/examples/scaledown/scaledown-test-near.png)
 
 * `Gaussian`:

    ![Gaussian](https://raw.githubusercontent.com/image-rs/image/master/examples/scaledown/scaledown-test-gauss.png)

 * `Lanczos3`:

    ![Lanczos with window 3](https://raw.githubusercontent.com/image-rs/image/master/examples/scaledown/scaledown-test-lcz2.png)

## Hash size

Dimensions of the final hash, as width x height, in bits. A hash size of 8, 8 produces an 8 x 8 bit (8 byte) hash. Larger hash sizes take more time to compute as well as more memory, but aren???t necessarily better for comparing images. The best hash size depends on both the hash algorithm and the input dataset. If your images are mostly wide aspect ratio (landscape) then a larger width and a smaller height hash size may be preferable. Optimal values can really only be discovered empirically though.