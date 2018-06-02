# seven.rs

`seven` is an extremely small file processor. It's currently not finished, and API changes could happen at any minor version increment.

## Installation

```sh
cargo add seven
```

## Usage

Currently, the way you'd use it is somewhat like this:

```rust
extern crate seven;

use seven::{SimpleFile, create_middleware};

fn main() {
  seven::run(vec![
    create_middleware(|files: &mut Vec<SimpleFile>| {
      let file: &mut SimpleFile = &mut files[0];
      file.content = "test hello".to_string();
    }, Some("source"), Some("destination"))
  ]).unwrap()
}
```

This reads all files from a directory, and replaces the content of the first one with "test hello". That's about all it can do for now!

## License

Licensed under the AGPL-3.0+. See [LICENSE](./LICENSE).
