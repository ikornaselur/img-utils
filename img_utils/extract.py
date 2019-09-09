from img_utils import img_utils as _lib

from .utils import u8


def extract_blues(src_path: str, dst_path: str, min_diff: int, min_blue: int):
    """ Extract Blues

    `min_diff` and `min_blue` are clamped between (inclusive) 0-255

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
    """
    _lib._extract_blues(src_path, dst_path, u8(min_diff), u8(min_blue))
