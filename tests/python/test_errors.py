import tempfile

import img_utils  # type: ignore


def test_darken_pixels_file_not_found():
    img_utils.darken_pixels("/file.jpg", "/out.jpg", 0, 0)


def test_darken_pixels_invalid_u8_arguments():
    out = tempfile.NamedTemporaryFile(suffix=".jpg").name

    img_utils.darken_pixels("./tests/assets/test-small.jpg", out, -10, 25)
