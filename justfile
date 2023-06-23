#!/usr/bin/just

test:
	cargo test --all-features

docs: get-deps-book
	mdbook build docs -d book
	cd book && git init
	cd book && git config --global user.name "${CI_REPO_OWNER}" user.email "${MAIL}"
	cd book && git remote add origin "https://${RYPPER_ACCESS_TOKEN}@codeberg.org/${CI_REPO}.git"
	cd book && git switch --orphan pages
	cd book && git add -A
	cd book && git commit -m "update book for commit ${CI_COMMIT_SHA}"
	cd book && git push --force -u origin pages

get-deps:
	zypper --non-interactive install gcc gcc-c++ cargo libnettle-devel libzstd-devel

get-deps-book:
	zypper --non-interactive install mdbook git

build-release:
	cargo build --release --all-features

install: get-deps build-release
	cargo install --path .

publish:
	cargo publish --token "${CARGO_REGISTRY_TOKEN}"

# Ignore install section
do-all: get-deps build-release test docs publish
