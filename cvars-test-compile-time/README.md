A dummy crate that can be compiled into a binary like a normal game but only contains lots of cvars so we can measure how long the proc macros take to run and therefore how using them affects compile times of games.

```zsh
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features nomacro"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars100,setgetdummy"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars1000,setgetdummy"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars10000,setgetdummy"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars100,setget"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars1000,setget"
hyperfine --show-output --warmup 2 "echo '// test' >> src/main.rs && time ~/dev/cpp/mold/mold -run cargo build --features cvars10000,setget"
```
