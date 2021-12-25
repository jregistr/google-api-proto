#!/usr/bin/env bash
set -u
set +e

cd $(dirname ${BASH_SOURCE:-$0})

echo "Update git submodule"
git submodule update --remote --rebase xtask/proto

echo "Run xtask all"
cargo xtask all

if git diff --exit-code --quiet; then
  echo "No changes!"
else
  if [ ${CI:-false} ]; then
    git config --local user.name "mechiru"
    git config --local user.email "mechiru@users.noreply.github.com"
  fi

  git add xtask/proto \
    && git commit -m "xtask: update git submodule googleapis/googleapis"
  git add google-api-proto \
    && git commit -m "google-api-proto: regenerate code"

  echo "Sync with the origin repository"
  git push -u origin master

  echo "Publish to crates.io"
  cargo release \
        --execute \
        --no-confirm \
        --no-publish \
        --package google-api-proto \
        --token "$CARGO_API_TOKEN" \
        --verbose \
        alpha
fi
