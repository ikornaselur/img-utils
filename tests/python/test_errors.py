import tempfile

import mock
import pytest  # type: ignore
from img_utils import darken_pixels
from img_utils.exceptions import (FileNotFoundException,
                                  UnsupportedFormatException)


def test_darken_pixels_file_not_found():
    with pytest.raises(FileNotFoundException):
        darken_pixels("/non/existant/file.jpg", "/out.jpg", 0, 0)


def test_darken_pixels_invalid_file_type():
    with pytest.raises(UnsupportedFormatException):
        darken_pixels("./tests/assets/bin", "/out.jpg", 0, 0)


def test_darken_pixels_invalid_u8_arguments():
    out = tempfile.NamedTemporaryFile(suffix=".jpg").name

    darken_pixels("./tests/assets/test-small.jpg", out, -10, 25)
