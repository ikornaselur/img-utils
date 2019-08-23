build:
	cargo build
	mv target/debug/libimg_utils.dylib img_utils.so

build_release:
	cargo build --release
	mv target/release/libimg_utils.dylib img_utils.so

clean:
	cargo clean
	rm img_utils.so

run: build
	python main.py

run_release: build_release
	python main.py
