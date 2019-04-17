# ellipse

[![Travis CI Status](https://travis-ci.org/NeoLegends/ellipse.svg?branch=master)](https://travis-ci.org/NeoLegends/ellipse)
[![crates.io](https://img.shields.io/crates/v/ellipse.svg)](https://crates.io/crates/ellipse)
[![Documentation](https://docs.rs/ellipse/badge.svg)](https://docs.rs/ellipse)

Truncate and ellipse strings in a human-friendly way. Uses unicode extended grapheme clusters.

[Documentation](https://docs.rs/ellipse)

## Example

```rust
use ellipse::Ellipse;

let input = "🇩🇪🇬🇧🇮🇹🇫🇷";
assert_eq!(&input.truncate_ellipse(2), "🇩🇪🇬🇧...");
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
