extern crate walkdir;

use std::fs::DirBuilder;
use std::fs::File;
use std::ffi::OsString;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct SimpleFile {
    pub name: OsString,
    pub content: String,
    pub abs_path: PathBuf,
    pub rel_path: PathBuf,
}

type MiddlewareFunction = Box<FnMut(&mut Vec<SimpleFile>)>;

pub fn create_middleware<T>(x: T) -> Box<T> {
    Box::new(x)
}

pub fn seven(
    middleware: Vec<MiddlewareFunction>,
    source: Option<&str>,
    destination: Option<&str>,
) -> Result<Vec<SimpleFile>, io::Error> {
    let f_source = source.unwrap_or(".");
    let f_dest = destination.unwrap_or("_site");
    let mut files = Vec::<SimpleFile>::new();
    read_dir(&mut files, f_source)?;
    for mut function in middleware {
        function(&mut files);
    }
    write_dir(&mut files, f_source, f_dest)?;
    Ok(files)
}

fn read_dir(files: &mut Vec<SimpleFile>, source: &str) -> Result<(), io::Error> {
    for entry in WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path().to_owned();
        if !&path.is_dir() {
            let mut file = File::open(&path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let file_struct = SimpleFile {
                name: path.clone().file_name().unwrap().to_os_string(),
                content,
                abs_path: path.clone().canonicalize()?,
                rel_path: path,
            };
            &files.push(file_struct);
        }
    }
    Ok(())
}

fn write_dir(files: &mut Vec<SimpleFile>, source: &str, destination: &str) -> Result<(), io::Error> {
    for file in files {
        let temp_path = file.rel_path.strip_prefix(source).unwrap();
        let destination_path = PathBuf::from(destination).join(temp_path);
        let mut dir_path = destination_path.clone();
        dir_path.pop();
        DirBuilder::new().recursive(true).create(&dir_path)?;
        let mut fileref = File::create(&destination_path)?;
        fileref.write_all(file.content.as_bytes())?;
    }
    Ok(())
}
