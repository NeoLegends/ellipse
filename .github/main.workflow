workflow "CI" {
  resolves = ["Rust Action"]
  on = "push"
}

action "Rust Action" {
  uses = "icepuma/rust-action@1.0.6"
}
