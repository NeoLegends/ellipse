workflow "CI" {
  resolves = ["Rust Action"]
  on = "push"
}

action "Rust Action" {
  uses = "icepuma/rust-action@master"
  args = "cargo fmt -- --check && cargo clippy -- -Dwarnings && cargo test"
}
