clean:
	cargo clean
	rm -f *.so
	rm -f *.jpg

pipenv:
ifeq (, $(shell which pipenv))
	pip install pipenv
endif

venv: pipenv
	pipenv install --dev
	ln -s $(shell pipenv --venv) venv

bench: venv
	echo "Running benchmark with debug build"
	cargo build --release
	mv target/debug/libimg_utils.dylib img_utils.so
	pipenv run python main.py


########################
# PyPI publish targets #
########################
publish_macos:
	cargo clean
	echo "Publishing for python 3.5, 3.6 and 3.7 on macos"
	maturin publish \
		-u "${PYPI_USERNAME}" \
		-p "${PYPI_PASSWORD}" \
		-i python3.5 \
		-i python3.6 \
		-i python3.7

publish_linux:
	echo "Publishing for python 3.5, 3.6 and 3.7 on linux"
	docker run --rm -v `pwd`:/io konstin2/maturin publish \
		-u "${PYPI_USERNAME}" \
		-p "${PYPI_PASSWORD}" \
		-i python3.5 \
		-i python3.6 \
		-i python3.7

publish: publish_macos publish_linux
