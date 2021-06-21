# filenamify

[![crates-svg]][crates-url]
[![deps-svg]][deps-url]

[crates-svg]: https://img.shields.io/crates/v/filenamify.svg
[crates-url]: https://crates.io/crates/filenamify
[deps-svg]: https://deps.rs/repo/github/chawyehsu/filenamify-rs/status.svg
[deps-url]: https://deps.rs/repo/github/chawyehsu/filenamify-rs

> Convert a string to a valid safe filename

## Install

```toml
[dependencies]
filenamify = "0.1"
```

## Examples

```rust
use filenamify::filenamify;
let safe_filename = filenamify("//foo/bar/file");
println!("{}", safe_filename); // Prints "_foo_bar_file"
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

<sub>
Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
</sub>
