# nya.rs [![Build Status](https://travis-ci.org/komaeda/nya.svg?branch=master)](https://travis-ci.org/komaeda/nya)

`nya` is a small and cute file processor. It does only what you want it to do, and nothing else.

## Installation

Make sure you have Rust 2018 (Rust 1.31+) installed.

```sh
cargo add nya
```

Install [cargo-edit](https://github.com/killercup/cargo-edit) to extend Cargo, allowing you to add, remove, and upgrade dependencies by modifying your Cargo.toml file from the command line.

## Usage

You'd use `nya` somewhat like this:

```rust
extern crate nya;

use nya::{ignore, create_middleware};

fn main() {
  nya::run(vec![
    ignore(vec!["target/", ".DS_Store"]),
    create_middleware(|files| {
      let file = &mut files[0];
      file.content = "test hello".to_string();
    }, Some("source"), Some("destination"))
  ]).unwrap()
}
```

This reads all files from a directory, and replaces the content of the first one with "test hello". Full documentation can be found on [docs.rs](http://docs.rs/nya).

## License

Licensed under the AGPL-3.0+. See [LICENSE](./LICENSE).
