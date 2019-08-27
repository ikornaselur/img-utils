build:
	cargo build
	mv target/debug/libimg_utils.dylib img_utils.so

build_release:
	cargo build --release
	mv target/release/libimg_utils.dylib img_utils.so

clean:
	cargo clean
	rm *.so 2>/dev/null
	rm *.jpg 2>/dev/null

venv:
	echo "Setting up virtualenv with Pillow"
	virtualenv --python=python3.7 venv
	venv/bin/pip install Pillow==6.1.0

bench: build venv
	echo "Running benchmark with debug build"
	venv/bin/python main.py

bench_release: build_release venv
	echo "Running benchmark with debug release"
	venv/bin/python main.py

publish_macos:
	cargo clean
	echo "Publishing for python 3.5, 3.6 and 3.7 on macos"
	pyo3-pack publish \
		-u "${PYPI_USERNAME}" \
		-p "${PYPI_PASSWORD}" \
		-i python3.5 \
		-i python3.6 \
		-i python3.7

publish_linux:
	echo "Publishing for python 3.5, 3.6 and 3.7 on linux"
	docker run --rm -v `pwd`:/io konstin2/pyo3-pack publish \
		-u "${PYPI_USERNAME}" \
		-p "${PYPI_PASSWORD}" \
		-i python3.5 \
		-i python3.6 \
		-i python3.7

publish: publish_macos publish_linux
