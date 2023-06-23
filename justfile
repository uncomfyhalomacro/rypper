#!/usr/bin/just

test:
	cargo test --all-features

docs: get-deps-book
	echo "Building docs..."
	mdbook build docs -d book
	echo "Docs built."
	echo "Entering book directory..."
	cd book
	git init
	git config --global user.name "${CI_REPO_OWNER}" user.email "${MAIL}"
	git remote add origin "https://${RYPPER_ACCESS_TOKEN}@codeberg.org/${CI_REPO}.git"
	git switch --orphan pages
	git add -A
	git commit -m "update book for commit ${CI_COMMIT_SHA}"
	git push --force -u origin pages
	cd ../

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

