#!/usr/bin/just

test:
	cargo test --all-features

docs: get-deps-book
    #!/usr/bin/bash
	set -euxo pipefail
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

get-deps:
	zypper --non-interactive install gcc gcc-c++ cargo libnettle-devel libzstd-devel

get-deps-book:
	zypper --non-interactive install mdbook git

build-release:
	echo "Building rypper..."
	cargo build --release --all-features
	echo "Rypper built."

install: get-deps build-release
	cargo install --path .

publish:
	cargo publish --token "${CARGO_REGISTRY_TOKEN}"

# Ignore install section
do-all: get-deps build-release test docs publish
