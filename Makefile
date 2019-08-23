build:
	cargo build
	mv target/debug/libimg_utils.dylib img_utils.so

clean:
	cargo clean
	rm img_utils.so
