#!/usr/bin/env bash

# Exit on error, unset variables and pipeline errors
set -euo pipefail

hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features nomacro"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars100,derive-dummy"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars1000,derive-dummy"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars10000,derive-dummy"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars100,derive"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars1000,derive"
#hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars10000,derive" # TODO this gets stuck
