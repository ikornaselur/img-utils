import timeit

import img_utils
from PIL import Image

FILE_NAME = "./example.jpg"
ARGS = (FILE_NAME, 80, 220)


def rust_impl(file_name, amount=75, cutoff=220):
    img_utils.increase_contrast(file_name, amount, cutoff)


def python_impl(file_name, amount=75, cutoff=220):
    original = Image.open(file_name)
    original_pixels = original.load()

    new_image = Image.new(original.mode, original.size)
    new_pixels = new_image.load()

    for i in range(new_image.size[0]):
        for j in range(new_image.size[1]):
            (r, g, b) = original_pixels[i, j]
            lower = min(r, g, b) < cutoff
            new_pixels[i, j] = (
                r - int(r * amount / 100) if lower else r,
                g - int(g * amount / 100) if lower else g,
                b - int(b * amount / 100) if lower else b,
            )

    with open("out-python.jpg", "wb") as f:
        new_image.save(f, format=original.format)


if __name__ == "__main__":

    def pyth():
        python_impl(FILE_NAME)

    py_runs = 3
    total = timeit.timeit(pyth, number=py_runs)
    print(f"Python: {total/py_runs * 1000:.02f} ms avg")

    def rust():
        rust_impl(FILE_NAME)

    ru_runs = 10
    total = timeit.timeit(rust, number=ru_runs)
    print(f"Rust: {total/ru_runs * 1000:.02f} ms avg")
