#!/usr/bin/env zsh

cargo run --release -- --clear-all-outputs README.md

cargo run --release -- \
  -s "Usage" \
  -s "Another couple of examples" \
  -s "Some section" \
  -s "Another section" \
  README.md
