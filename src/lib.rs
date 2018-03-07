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

type MiddlewareFunction = Box<FnMut(&mut Vec<SimpleFile>)>;

pub fn seven(middleware: Vec<MiddlewareFunction>) -> () {
    let mut files = Vec::<SimpleFile>::new();
    read_dir(&mut files);
    for mut function in middleware {
        function(&mut files);
    }
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
    seven(vec![Box::new(|files: &mut Vec<SimpleFile>| {
        let file: &mut SimpleFile = &mut files[0];
        file.content = "test hello".to_string();
    }), Box::new(|files: &mut Vec<SimpleFile>| {
        let file: &mut SimpleFile = &mut files[0];
        file.content = "override".to_string();
    })]);
}
