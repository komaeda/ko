extern crate walkdir;

use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct SimpleFile {
    content: String,
    path: PathBuf,
}

pub fn seven() {
    let mut files = Vec::<SimpleFile>::new();
    read_dir(&mut files);
    println!("{:?}", files);
}

fn read_dir(files: &mut Vec<SimpleFile>) {
    for entry in WalkDir::new("example") {
        let entry = entry.unwrap();
        let path = entry.path().to_owned();
        if !&path.is_dir() {
            let mut file = File::open(&path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            let file_struct = SimpleFile {
                content: content,
                path: path,
            };
            &files.push(file_struct);
        }
    }
}
