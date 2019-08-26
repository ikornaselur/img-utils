## Image utils

Port of some slow functions from Python to a Rust library with Python bindings, mainly to experiment with [rust-cpython](https://github.com/dgrunwald/rust-cpython)

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
