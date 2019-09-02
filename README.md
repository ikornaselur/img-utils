# Rusty Image Utils

Port of some slow functions from Python to a Rust library with Python bindings,
mainly to experiment with [rust-cpython](https://github.com/dgrunwald/rust-cpython)

Published as [`rusty-img-utils`](https://pypi.org/project/rusty-img-utils/) on pypi for Python 3.5, 3.6 and 3.7 on linux and
macos

## Functions

### `darken_pixels`

Darkens all pixels in the image by percentage, specified by `amount`. Any pixel
that doesn't have a subpixel below than the `cutoff` will be ignored.

```python
import img_utils
img_utils.darken_pixels(
    src_path="in_file.jpg",
    dst_path="out_file.jpg",
    amount=80,
    cutoff=200,
)
```

will take the `in_file.jpg` and lower each subpixel of the image by 80%, unless
all the subpixels are above 200.

The RGB pixel `100, 220, 220` will be turned into `20, 44, 44` while `210, 220,
230` will be left alone.

### `extract_blues`

Selects all pixels that have the blue pixel above a certain limit and either
green or red the minimum difference blow the blue pixel. The selected pixels
will be output as completely black while any other pixel is output as white.

```python
import img_utils
img_utils.extract_blues(
    src_path="in_file.jpg",
    dst_path="out_file.jpg",
    min_diff=30,
    min_blue=100,
)
```

will take the `in_file.jpg` and any pixel that has the blue pixel 100 or higher
and either red or green pixel at 70 or lower (100 - 30) will be black, all
others will be white.


## Performance comparison

Testing with 100x100 and 1000x1000 random noise images on a 2018 MBP

### `darken_pixels`
```
*** Testing small size
Python_Darken: 12.14 ms avg (412 runs)
Rust_Darken: 1.15 ms avg (4357 runs)

Rust is 10.58x faster for small size

*** Testing normal size
Python_Darken: 1103.74 ms avg (5 runs)
Rust_Darken: 46.41 ms avg (108 runs)

Rust is 23.78x faster for normal size
```

### `extract_blues`
```
*** Testing small size
Python_Extract: 4.94 ms avg (1012 runs)
Rust_Extract: 1.17 ms avg (4282 runs)

Rust is 4.23x faster for small size

*** Testing normal size
Python_Extract: 392.25 ms avg (13 runs)
Rust_Extract: 47.71 ms avg (105 runs)

Rust is 8.22x faster for normal size
```
