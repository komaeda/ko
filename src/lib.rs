//! `nya` is a simple file processor. It's extremely simple, very fast, and super
//! cute, as well. It reads files from a directory, does stuff to them, and then
//! spits them out somewhere else. That's all it does. The "stuff" that's being
//! done to the files is called _middleware_. Middleware is just a function that
//! takes a struct representing a file and then does something to that file.
//! It doesn't even have to return it! That's how easy it is to use `nya`. Can
//! you believe it?
//!
//! In its simplest form, you'd use it somewhat like this:
//!
//! ```
//! extern crate nya;
//!
//! use nya::create_middleware;
//!
//! fn main() {
//!     nya::run(vec![
//!         create_middleware(|files| {
//!             let file = &mut files[0];
//!             file.content = "test hello".to_string();
//!         }, Some("source"), Some("destination"))
//!     ]).unwrap()
//! }
//! ```
//!
//! ### How does `nya` compare to other software?
//!
//! At some distant point in time, I'd like for `nya` to be used as a static site
//! generator. How would it compare to other existing static site generators, then?
//! Well, it's much simpler than __Jekyll__ or __Hugo__, for example, and it's even
//! simpler than other JavaScript-based SSGs such as __Metalsmith__ or __Wintersmith__.
//! Existing Rust SSGs have all more or less tried to replicate the full feature set
//! of Jekyll or comparable software, so I hope I'll be scratching an itch here.
//!
//! `nya` currently only depends on `walkdir`, and I'd like to keep dependencies
//! as light as possible.

extern crate walkdir;

use std::fs::DirBuilder;
use std::fs::File;
use std::ffi::OsString;
use std::io::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

/// A struct describing a simple file, with only a name, content, and its path.
///
/// # Examples
///
/// ```
/// let file = nya::SimpleFile {
///     name: OsString::from("coolfile.txt"),
///     content: "hello".to_string(),
///     abs_path: PathBuf::from(r"/home/coolfile.txt"),
///     rel_path: PathBuf::from(r"coolfile.txt"),
/// }
/// ```
#[derive(Debug)]
pub struct SimpleFile {
    /// The filename, as an `OsString`.
    pub name: OsString,
    /// The content of the file, as an owned `String`.
    pub content: String,
    /// The absolute path of the file, as a `PathBuf`.
    pub abs_path: PathBuf,
    /// The relative path of the file, as a `PathBuf`.
    pub rel_path: PathBuf,
}

type MiddlewareFunction = Box<FnMut(&mut Vec<SimpleFile>)>;

/// A convenience function that creates middleware.
///
/// Takes a closure and boxes it so it can be
/// (more or less) safely added to the middleware chain and so that we don't
/// have to worry about lifetimes. This closure itself takes a `files` argument
/// that is a `Vec` of `SimpleFile`s. This Vec can be modified in place and
/// doesn't have to be returned.
///
/// # Example
///
/// ```
/// let func = nya::create_middleware(|files| {
///     let file = files[0];
///     file.content = "haha hello";
/// }
///
/// nya::run(vec![func]).unwrap();
/// ```
pub fn create_middleware<T>(x: T) -> Box<T> {
    Box::new(x)
}

/// Runs a middleware chain.
///
/// Reads a file from the `source` argument (default is the
/// current directory), runs the collected files through the provided middleware
/// chain, and then writes the result to the `destination` argument (default
/// is `_site`). Both arguments _have_ to be provided, although they accept
/// an `Option`, so you can pass in `None` if you want the defaults to apply.
/// Returns a `Result<Vec<SimpleFile>, std::io::Error>`.
///
/// # Example
///
/// ```
/// let func = nya::create_middleware(|files| {
///     let file = files[0];
///     file.content = "haha hello";
/// }
///
/// let result = nya::run(vec![func]);
/// if let Ok(r) = result {
///     println!("Success!");
/// }
/// ```

pub fn run(
    middleware: Vec<MiddlewareFunction>,
    source: Option<&str>,
    destination: Option<&str>,
) -> Result<Vec<SimpleFile>, std::io::Error> {
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

fn read_dir(files: &mut Vec<SimpleFile>, source: &str) -> Result<(), std::io::Error> {
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

fn write_dir(files: &mut Vec<SimpleFile>, source: &str, destination: &str) -> Result<(), std::io::Error> {
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
