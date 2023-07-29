#!/usr/bin/just

docs: get-deps-book
    #!/usr/bin/env bash
    set -euxo pipefail
    PATH="${PATH}:${PWD}/mdbook-deps/bin"
    export PATH
    pushd docs
    mdbook-mermaid install
    mdbook-catppuccin install
    mdbook build
    popd
    git config --global init.defaultBranch main
    git config --global user.name "${CI_REPO_OWNER}" 
    git config --global user.email "${MAIL}"
    pushd ../
    mkdir -p docs/
    pushd docs
    git init 
    git remote add origin "https://${RYPPER_ACCESS_TOKEN}@codeberg.org/${CI_REPO}.git"
    git switch --orphan pages
    cp ../${CI_REPO_NAME}/.domains .domains
    if [ -n "${CI_COMMIT_TAG:-}" ]; then
      cp -rfv ../${CI_REPO_NAME}/docs/book/* .
      cp -rfv ../${CI_REPO_NAME}/docs/book stable
      git add -A
      git commit -m "update book for ${CI_COMMIT_TAG:-}"
    fi
    cp -rfv ../${CI_REPO_NAME}/docs/book dev
    git add -A
    git commit -m "update book for commit ${CI_COMMIT_SHA}"
    git push --force -u origin pages
    popd

get-deps:
    zypper --non-interactive install gcc gcc-c++ cargo libnettle-devel libzstd-devel libopenssl-devel clang-devel

get-deps-book:
    zypper --non-interactive install mdbook git cargo
    mkdir -p mdbook-deps/bin
    cargo install --git "https://github.com/catppuccin/mdBook" --root mdbook-deps
    cargo install mdbook-mermaid --root mdbook-deps

build:
    cargo build

test:
    cargo test

build-release:
    cargo build --release --all-features

test-release:
    cargo test --release --all-features

install: get-deps build-release
    cargo install --path .

publish:
    cargo publish --token "${CARGO_REGISTRY_TOKEN}"

format:
    cargo fmt

# Ignore install section
do-all: get-deps build-release test docs publish
