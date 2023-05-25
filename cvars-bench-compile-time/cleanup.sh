#!/usr/bin/env bash

# Cleanup after running the benchmarks.
# This is a separate script for convenience.

sed --in-place 's/test0a*/test0/' src/*.in

grep --invert-match '^// test$' src/main.rs > src/main.rs.tmp
mv src/main.rs.tmp src/main.rs
