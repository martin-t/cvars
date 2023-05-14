#!/usr/bin/env bash

# Exit on error, unset variables and pipeline errors
set -euo pipefail

# In case this is the first time building this project,
# this'll also build deps which can take a while.
# Run it outside hyperfine so we can see progress.
cargo build --features nomacro,cvars-100

hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features nomacro,cvars-100"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features nomacro,cvars-1000"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features nomacro,cvars-10000"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features nomacro,cvars-10000d"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features derive-dummy,cvars-100"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features derive-dummy,cvars-1000"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features derive-dummy,cvars-10000"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features derive-dummy,cvars-10000d"
# hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features derive,cvars-100"
# hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features derive,cvars-1000"
# #hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features derive,cvars-10000" # TODO this gets stuck
# hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features fnlike,cvars-100"
# hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features fnlike,cvars-1000"
# #hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features fnlike,cvars-10000" # TODO this gets stuck

# Cleanup
grep --invert-match '^// test$' src/main.rs > src/main.rs.tmp
mv src/main.rs.tmp src/main.rs
