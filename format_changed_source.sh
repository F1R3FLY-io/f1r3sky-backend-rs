#!/bin/bash

# Format all staged Rust files with rustfmt
STAGED_RS_FILES=$(git diff --name-only | grep '\.rs$')
echo "Staged Rust files: $STAGED_RS_FILES"

if [ -n "$STAGED_RS_FILES" ]; then
  echo "Formatting staged Rust files by $RUST_CMD..."
  for file in $STAGED_RS_FILES; do
    echo "Formatting Rust $file..."
    rustup run nightly rustfmt "$file"
    git add "$file"
  done
fi

STAGED_TOML_FILES=$(git diff --name-only | grep '\.toml$')
echo "Staged TOML files: $STAGED_TOML_FILES"

if [ -n "$STAGED_TOML_FILES" ]; then
  echo "Formatting staged TOML files  by $TAPLO_CMD..."
  for file in $STAGED_TOML_FILES; do
    echo "Formatting TOML $file..."
    taplo fmt "$file"
    git add "$file"
  done
fi

exit 0
