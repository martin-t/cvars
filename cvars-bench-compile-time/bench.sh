#!/usr/bin/env bash

# Benchmark the compile time of a crate using cvars.

# We're intentionally compiling in debug mode
# because we're interested in how using cvars affects incremental recompiles in gamedev.
# and the best option for gamedev is to optimize deps but not the gamecode in the root crate
# to test ideas quickly while still getting reasonable performance.

# Exit on error, unset variables and pipeline errors
set -euo pipefail

# In case this is the first time building this project,
# this'll also build deps which can take a while.
# Run it outside hyperfine so we can see progress.
cargo build --features nomacro,cvars-100

# Hyperfine supports multiple benchmarks in one command and a cleanup command
# but let's keep everything separate so we can comment some benches out
# and so we can easily copy-paste the commands to run them manually.

# TODO 10k cvars take several minutes, uncomment benchmarks when this is fixed.

# Measure incremental rebuild time after editing the Cvars struct.
hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/nomacro-100.in && cargo build --features string,typed,nomacro,cvars-100"
hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/nomacro-1000.in && cargo build --features string,typed,nomacro,cvars-1000"
hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/nomacro-10000.in && cargo build --features string,typed,nomacro,cvars-10000"
hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/derive-100.in && cargo build --features string,typed,derive-dummy,cvars-100"
hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/derive-1000.in && cargo build --features string,typed,derive-dummy,cvars-1000"
hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/derive-10000.in && cargo build --features string,typed,derive-dummy,cvars-10000"
hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/derive-100.in && cargo build --features string,typed,derive,cvars-100"
hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/derive-1000.in && cargo build --features string,typed,derive,cvars-1000"
#hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/derive-10000.in && cargo build --features string,typed,derive,cvars-10000"
hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/fnlike-100.in && cargo build --features string,typed,fnlike,cvars-100"
hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/fnlike-1000.in && cargo build --features string,typed,fnlike,cvars-1000"
#hyperfine --warmup 2 "sed --in-place 's/test0/test0a/' src/fnlike-10000.in && cargo build --features string,typed,fnlike,cvars-10000"

# Measure incremental rebuild time after editing main.rs.
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,nomacro,cvars-100"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,nomacro,cvars-1000"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,nomacro,cvars-10000"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,derive-dummy,cvars-100"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,derive-dummy,cvars-1000"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,derive-dummy,cvars-10000"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,derive,cvars-100"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,derive,cvars-1000"
#hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,derive,cvars-10000"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,fnlike,cvars-100"
hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,fnlike,cvars-1000"
#hyperfine --warmup 2 "echo '// test' >> src/main.rs && cargo build --features string,typed,fnlike,cvars-10000"

./cleanup.sh
