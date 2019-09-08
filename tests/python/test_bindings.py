import img_utils
import mock


def test_darken_pixels_calls_library():
    with mock.patch("img_utils.img_utils._darken_pixels") as mock_fun:
        img_utils.darken_pixels("foo", "bar", 0, 1)

    mock_fun.assert_called_with("foo", "bar", 0, 1)


def test_extract_blues_calls_library():
    with mock.patch("img_utils.img_utils._extract_blues") as mock_fun:
        img_utils.extract_blues("foo", "bar", 0, 1)

    mock_fun.assert_called_with("foo", "bar", 0, 1)
