def clamp(lower: int, value: int, upper: int) -> int:
    """ Clamp a value between (inclusive) lower and upper """
    return min(max(value, lower), upper)


def u8(value: int) -> int:
    """ Clamp value to the range 0-255 """
    return clamp(0, value, 255)
