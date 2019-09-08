clean:
	cargo clean
	pipenv clean
	rm -f *.so
	rm -f img_utils/*.so
	rm -f *.jpg

_pipenv:
ifeq (, $(shell which pipenv))
	pip install pipenv
endif

_venv:
	pipenv sync --dev

_link:
	rm -rf venv
	ln -s $(shell pipenv --venv) venv

# Set up environment with pipenv
#
# pipenv --venv returns empty if run in the same target as initial sync for
# some reason, so split it up to two targets to run together
venv: _pipenv _venv _link

bench: venv
	echo "Running benchmark with debug build"
	cargo build --release
	mv target/debug/libimg_utils.dylib img_utils.so
	pipenv run python main.py

#########
# Tests #
#########
python_tests:
	pipenv run maturin develop
	pipenv run py.test tests/python

rust_tests:
	cargo test

tests: \
	rust_tests \
	python_tests


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
