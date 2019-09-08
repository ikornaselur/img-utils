from contextlib import contextmanager


class ImgUtilsException(Exception):
    pass


class FileNotFoundException(Exception):
    pass


class UnsupportedFormatException(Exception):
    pass


@contextmanager
def known_exceptions():
    """ Raise specific Python exceptions for known error codes, from errors.rs """
    try:
        yield
    except RuntimeError as e:
        if not len(e.args):
            raise e
        if e.args[0] == 1001:
            raise FileNotFoundException()
        if e.args[0] == 1002:
            raise UnsupportedFormatException()
        if e.args[0] == 9001:  # Unknown exception from the library
            raise ImgUtilsException()
        raise e
