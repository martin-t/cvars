#!/usr/bin/env bash

# Cleanup after running the benchmarks.
# This is a separate script for convenience.

grep --invert-match '^// test$' src/main.rs > src/main.rs.tmp
mv src/main.rs.tmp src/main.rs
