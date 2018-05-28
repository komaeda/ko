extern crate seven;

use std::fs::File;
use std::io::prelude::*;
use std::ffi::OsString;
use seven::SimpleFile;

#[test]
fn it_works() {
    let result = seven::seven(vec![
        Box::new(|files: &mut Vec<SimpleFile>| {
            let file: &mut SimpleFile = &mut files[0];
            file.content = "test hello".to_string();
        }),
        Box::new(|files: &mut Vec<SimpleFile>| {
            let file: &mut SimpleFile = &mut files[0];
            file.content = "override".to_string();
        }),
    ], Some("example"), None);
    assert_eq!(result[0].content, "override");
}

#[test]
fn custom_source() {
    let result = seven::seven(vec![
        Box::new(|files: &mut Vec<SimpleFile>| {
            let file: &mut SimpleFile = &mut files[0];
            file.content = "another test".to_string();
        }),
    ], Some("fixtures/custom_source"), None);
    assert_eq!(result[0].name, OsString::from("a.md"));
}
