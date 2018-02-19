use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn seven() {
    let path = Path::new("test.md");
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file: {}", why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read file: {}", why.description()),
        Ok(_) => print!("file contains: {}", s),
    };
}

#[test]
fn basic_test() {
    seven()
}
