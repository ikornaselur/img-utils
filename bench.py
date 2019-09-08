import time

import img_utils
from PIL import Image  # type: ignore

FILES = {"small": "test-small.jpg", "normal": "test-normal.jpg"}

MAX_RUNTIME = 5  # seconds

# Darken pixels
AMOUNT = 75
CUTOFF = 127

# Extract blues
MIN_DIFF = 50
MIN_BLUE = 100


def python_impl_darken(in_file: str, out_file: str, amount: int, cutoff: int):
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


def python_impl_extract(in_file: str, out_file: str, min_diff: int, min_blue: int):
    original = Image.open(in_file)
    original_pixels = original.load()

    new_image = Image.new(original.mode, original.size)
    new_pixels = new_image.load()

    for i in range(new_image.size[0]):
        for j in range(new_image.size[1]):
            (r, g, b) = original_pixels[i, j]
            if (b - r > min_diff or b - g > min_diff) and b > min_blue:
                new_pixels[i, j] = (0, 0, 0)
            else:
                new_pixels[i, j] = (255, 255, 255)

    with open(out_file, "wb") as f:
        new_image.save(f, format=original.format)


def bench(func, args, max_runtime=MAX_RUNTIME):
    start = time.time()
    runs = 0
    while time.time() - start < max_runtime:
        func(*args)
        runs += 1
    total = time.time() - start
    avg = total / runs * 1000  # ms

    print(f"{func.__name__.title()}: {avg:.02f} ms avg ({runs} runs)")

    return avg


def bench_funcs(rust, python):
    for size, file in FILES.items():
        in_path = f"tests/assets/{file}"
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


def run():
    def rust_darken(in_file: str, out_file: str):
        img_utils.darken_pixels(in_file, out_file, AMOUNT, CUTOFF)

    def rust_extract(in_file: str, out_file: str):
        img_utils.extract_blues(in_file, out_file, MIN_DIFF, MIN_BLUE)

    def python_darken(in_file: str, out_file: str):
        python_impl_darken(in_file, out_file, AMOUNT, CUTOFF)

    def python_extract(in_file: str, out_file: str):
        python_impl_extract(in_file, out_file, MIN_DIFF, MIN_BLUE)

    bench_funcs(rust_darken, python_darken)
    bench_funcs(rust_extract, python_extract)


if __name__ == "__main__":
    run()
