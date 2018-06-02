extern crate seven;

use std::ffi::OsString;
use seven::{create_middleware, SimpleFile};

#[test]
fn it_works() {
    let result = seven::seven(vec![
        create_middleware(|files: &mut Vec<SimpleFile>| {
            let file: &mut SimpleFile = &mut files[0];
            file.content = "test hello".to_string();
        }),
        create_middleware(|files: &mut Vec<SimpleFile>| {
            let file: &mut SimpleFile = &mut files[0];
            file.content = "override".to_string();
        }),
    ], Some("example"), None);

    if let Ok(r) = result {
        assert_eq!(r[0].content, "override".to_string());
    }
}

#[test]
fn custom_source() {
    let result = seven::seven(vec![
        create_middleware(|files: &mut Vec<SimpleFile>| {
            let file: &mut SimpleFile = &mut files[0];
            file.content = "another test".to_string();
        }),
    ], Some("fixtures/custom_source"), None);
    if let Ok(r) = result {
        assert_eq!(r[0].name, OsString::from("a.md"));
    }
}
