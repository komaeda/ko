extern crate walkdir;

use std::fs::DirBuilder;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct SimpleFile {
    pub content: String,
    pub abs_path: PathBuf,
    pub rel_path: PathBuf,
}

type MiddlewareFunction = Box<FnMut(&mut Vec<SimpleFile>)>;

pub fn seven(
    middleware: Vec<MiddlewareFunction>,
    source: Option<&str>,
    destination: Option<&str>,
) -> Vec<SimpleFile> {
    let f_source = source.unwrap_or(".");
    let f_dest = destination.unwrap_or("_site");
    let mut files = Vec::<SimpleFile>::new();
    read_dir(&mut files, f_source);
    for mut function in middleware {
        function(&mut files);
    }
    write_dir(&mut files, f_source, f_dest);
    files
}

fn read_dir(files: &mut Vec<SimpleFile>, source: &str) {
    for entry in WalkDir::new(source) {
        let entry = entry.unwrap();
        let path = entry.path().to_owned();
        if !&path.is_dir() {
            let mut file = File::open(&path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            let file_struct = SimpleFile {
                content,
                abs_path: path.clone().canonicalize().unwrap(),
                rel_path: path,
            };
            &files.push(file_struct);
        }
    }
}

fn write_dir(files: &mut Vec<SimpleFile>, source: &str, destination: &str) {
    for file in files {
        let temp_path = file.rel_path.strip_prefix(source).unwrap();
        let destination_path = PathBuf::from(destination).join(temp_path);
        let mut dir_path = destination_path.clone();
        dir_path.pop();
        DirBuilder::new().recursive(true).create(&dir_path).unwrap();
        let mut fileref = File::create(&destination_path).unwrap();
        fileref.write_all(file.content.as_bytes()).unwrap();
    }
}
