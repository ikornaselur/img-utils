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

publish_macos:
	@cargo clean
	@echo "Publishing for python 3.5, 3.6 and 3.7 on macos"
	@pyo3-pack publish \
		-u "${PYPI_USERNAME}" \
		-p "${PYPI_PASSWORD}" \
		-i python3.5 \
		-i python3.6 \
		-i python3.7

publish_linux:
	@echo "Publishing for python 3.5, 3.6 and 3.7 on linux"
	@docker run --rm -v `pwd`:/io konstin2/pyo3-pack publish \
		-u "${PYPI_USERNAME}" \
		-p "${PYPI_PASSWORD}" \
		-i python3.5 \
		-i python3.6 \
		-i python3.7

publish: publish_macos publish_linux
