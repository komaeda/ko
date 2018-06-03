# nya.rs

`nya` is an extremely small file processor. It's currently not finished, and API changes could happen at any minor version increment.

## Installation

```sh
cargo add nya
```

## Usage

Currently, the way you'd use it is somewhat like this:

```rust
extern crate nya;

use nya::{SimpleFile, create_middleware};

fn main() {
  nya::run(vec![
    create_middleware(|files: &mut Vec<SimpleFile>| {
      let file = &mut files[0];
      file.content = "test hello".to_string();
    }, Some("source"), Some("destination"))
  ]).unwrap()
}
```

This reads all files from a directory, and replaces the content of the first one with "test hello". That's about all it can do for now!

## License

Licensed under the AGPL-3.0+. See [LICENSE](./LICENSE).
