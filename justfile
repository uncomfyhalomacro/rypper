#!/usr/bin/just

test:
  cargo test --all-features

docs: get-deps-book
  #!/usr/bin/env bash
  set -euxo pipefail
  mdbook build docs
  git config --global init.defaultBranch main
  git config --global user.name "${CI_REPO_OWNER}" 
  git config --global user.email "${MAIL}"
  cd ../
  mkdir -p docs/
  cd docs
  git init 
  git remote add origin "https://${RYPPER_ACCESS_TOKEN}@codeberg.org/${CI_REPO}.git"
  git switch --orphan pages
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

get-deps:
  zypper --non-interactive install gcc gcc-c++ cargo libnettle-devel libzstd-devel libopenssl-devel

get-deps-book:
  zypper --non-interactive install mdbook git

build:
  cargo build

build-release:
  cargo build --release --all-features

install: get-deps build-release
  cargo install --path .

publish:
  cargo publish --token "${CARGO_REGISTRY_TOKEN}"

# Ignore install section
do-all: get-deps build-release test docs publish
