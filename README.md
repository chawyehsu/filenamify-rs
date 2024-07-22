# filenamify

> Convert a string to a valid filename

[![cicd][cicd-badge]][cicd]
[![docs-svg]][docs-url]
[![crates-svg]][crates-url]
[![deps-svg]][deps-url]

[cicd-badge]: https://github.com/chawyehsu/filenamify-rs/workflows/CI/badge.svg
[cicd]: https://github.com/chawyehsu/filenamify-rs/actions/workflows/ci.yml
[docs-svg]: https://docs.rs/filenamify/badge.svg
[docs-url]: https://docs.rs/filenamify
[crates-svg]: https://img.shields.io/crates/v/filenamify.svg
[crates-url]: https://crates.io/crates/filenamify
[deps-svg]: https://deps.rs/repo/github/chawyehsu/filenamify-rs/status.svg
[deps-url]: https://deps.rs/repo/github/chawyehsu/filenamify-rs

## Install

```toml
[dependencies]
filenamify = "0.1"
```

## Examples

```rust
use filenamify::filenamify;
let safe_filename = filenamify("//foo/bar/file");
assert_eq!(safe_filename, "_foo_bar_file");
```

## License

**filenamify-rs** © [Chawye Hsu](https://github.com/chawyehsu). Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) license at your option.

> [Blog](https://chawyehsu.com) · GitHub [@chawyehsu](https://github.com/chawyehsu) · Twitter [@chawyehsu](https://twitter.com/chawyehsu)
