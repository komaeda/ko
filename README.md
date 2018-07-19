# nya.rs [![Build Status](https://travis-ci.org/isbr/nya.svg?branch=master)](https://travis-ci.org/isbr/nya)

`nya` is a small and cute file processor. It does only what you want it to do, and nothing else.

## Installation

```sh
cargo add nya
```

## Usage

Currently, the way you'd use it is somewhat like this:

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

This reads all files from a directory, and replaces the content of the first one with "test hello". Further documentation can be found on [docs.rs](http://docs.rs/nya).

## License

Licensed under the AGPL-3.0+. See [LICENSE](./LICENSE).
