build:
	@cargo build -q
	@mv target/debug/libimg_utils.dylib img_utils.so

build_release:
	@cargo build -q --release
	@mv target/release/libimg_utils.dylib img_utils.so

clean:
	cargo clean
	rm img_utils.so

bench: build
	@python main.py

bench_release: build_release
	@python main.py
