build:
	@cargo build -q
	@mv target/debug/libimg_utils.dylib img_utils.so

build_release:
	@cargo build -q --release
	@mv target/release/libimg_utils.dylib img_utils.so

clean:
	cargo clean
	rm *.so
	rm *.jpg

bench: build
	@venv/bin/python main.py

bench_release: build_release
	@venv/bin/python main.py
