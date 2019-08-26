import time

import img_utils
from PIL import Image

FILES = {"small": "test-small.jpg", "normal": "test-normal.jpg"}
AMOUNT = 75
CUTOFF = 127


def python_impl(in_file: str, out_file: str, amount: int, cutoff: int):
    original = Image.open(in_file)
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

    with open(out_file, "wb") as f:
        new_image.save(f, format=original.format)


if __name__ == "__main__":

    def bench(func, args, max_runtime=5):
        start = time.time()
        runs = 0
        while time.time() - start < max_runtime:
            func(*args)
            runs += 1
        total = time.time() - start
        avg = total / runs * 1000  # ms

        print(f"{func.__name__.title()}: {avg:.02f} ms avg ({runs} runs)")

        return avg

    def rust(in_file: str, out_file: str):
        img_utils.darken_pixels(in_file, out_file, AMOUNT, CUTOFF)

    def python(in_file: str, out_file: str):
        python_impl(in_file, out_file, AMOUNT, CUTOFF)

    for size, file in FILES.items():
        in_path = f"test-img/{file}"
        out_path = f"/tmp/{file}"

        print(f"*** Testing {size} size")
        py_avg = bench(python, args=[in_path, out_path])
        ru_avg = bench(rust, args=[in_path, out_path])
        print()

        diff = py_avg / ru_avg
        if diff > 1:
            print(f"Rust is {diff:.02f}x faster for {size} size")
        else:
            print(f"Rust is {1/diff:.02f}x slower for {size} size")
        print()
