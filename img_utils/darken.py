from img_utils import img_utils as _lib

from .utils import u8


def darken_pixels(src_path: str, dst_path: str, amount: int, cutoff: int):
    """ Darken Pixels

    Darkens all pixels in the image by percentage, specified by `amount`. Any pixel
    that doesn't have a subpixel below than the `cutoff` will be ignored.

    `amount` and `cutoff` are clamped between (inclusive) 0-255

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

    """
    _lib._darken_pixels(src_path, dst_path, u8(amount), u8(cutoff))
