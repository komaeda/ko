extern crate walkdir;

use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use walkdir::WalkDir;
use std::fs::DirBuilder;

#[derive(Debug)]
pub struct SimpleFile {
    content: String,
    abs_path: PathBuf,
    rel_path: PathBuf,
}

pub fn seven<F>(mut middleware: F) -> ()
    where F: FnMut(&mut Vec<SimpleFile>) -> &mut Vec<SimpleFile> {
    let mut files = Vec::<SimpleFile>::new();
    read_dir(&mut files);
    middleware(&mut files);
    write_dir(&mut files);
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
                abs_path: path.clone().canonicalize().unwrap(),
                rel_path: path,
            };
            &files.push(file_struct);
        }
    }
}

fn write_dir(files: &mut Vec<SimpleFile>) {
    for file in files {
        let temp_path = file.rel_path.strip_prefix("example").unwrap();
        let destination_path = PathBuf::from("destination").join(temp_path);
        let mut dir_path = destination_path.clone();
        dir_path.pop();
        DirBuilder::new().recursive(true).create(&dir_path).unwrap();
        let mut fileref = File::create(&destination_path).unwrap();
        fileref.write_all(file.content.as_bytes()).unwrap();
    }
}

#[test]
fn test () {
    seven(|files| {
        files[0].content = "hello".to_string();
        files
    });
}
