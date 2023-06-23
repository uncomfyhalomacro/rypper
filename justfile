#!/usr/bin/just

test:
	cargo test --all-features

docs: get-deps-book
	just docs/build

build-release:
	echo "Building rypper..."
	cargo build --release --all-features
	echo "Rypper built."

install: get-deps
	cargo install .

publish:
	cargo publish --token "${CARGO_REGISTRY_TOKEN}"

get-deps-book:
	zypper --non-interactive install mdbook git

get-deps:
	zypper --non-interactive install gcc gcc-c++ cargo libnettle-devel libzstd-devel

# Ignore install section
do-all: get-deps build-release test docs publish
