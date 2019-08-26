## Rusty Image Utils

Port of some slow functions from Python to a Rust library with Python bindings,
mainly to experiment with [rust-cpython](https://github.com/dgrunwald/rust-cpython)

Published as [`rusty-img-utils`](https://pypi.org/project/rusty-img-utils/) on pypi for Python 3.5, 3.6 and 3.7 on linux and
macos

### Functions

#### `darken_pixels`

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


### Performance comparison

Testing with 100x100 and 1000x1000 random noise images on a 2018 MBP

#### `darken_pixels`
```
*** Testing small size
Python: 12.46 ms avg (402 runs)
Rust: 1.10 ms avg (4534 runs)

Rust is 11.30x faster for small size

*** Testing normal size
Python: 1138.35 ms avg (5 runs)
Rust: 49.45 ms avg (102 runs)

Rust is 23.02x faster for normal size
```
