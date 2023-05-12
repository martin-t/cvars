#!/usr/bin/env bash

# Exit on error, unset variables and pipeline errors
set -euo pipefail

hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time cargo build --features nomacro"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time cargo build --features cvars100,derive-dummy"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time cargo build --features cvars1000,derive-dummy"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time cargo build --features cvars10000,derive-dummy"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time cargo build --features cvars100,derive"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time cargo build --features cvars1000,derive"
#hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time cargo build --features cvars10000,derive" # TODO this gets stuck

# Cleanup
grep --invert-match '^// test$' src/main.rs > src/main.rs.tmp
mv src/main.rs.tmp src/main.rs
