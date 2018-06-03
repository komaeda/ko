extern crate nya;

use std::ffi::OsString;
use std::path::Path;
use nya::{create_middleware, SimpleFile};

#[test]
fn it_works() {
    let result = nya::run(vec![
        create_middleware(|files: &mut Vec<SimpleFile>| {
            let file = &mut files[0];
            file.content = "test hello".to_string();
        }),
        create_middleware(|files: &mut Vec<SimpleFile>| {
            let file = &mut files[0];
            file.content = "override".to_string();
        }),
    ], Some("example"), None);

    if let Ok(r) = result {
        assert_eq!(r[0].content, "override".to_string());
    }
}

#[test]
fn custom_source() {
    let result = nya::run(vec![
        create_middleware(|files: &mut Vec<SimpleFile>| {
            let file = &mut files[0];
            file.content = "another test".to_string();
        }),
    ], Some("fixtures/custom_source"), None);
    if let Ok(r) = result {
        assert_eq!(r[0].name, OsString::from("a.md"));
    }
}

#[test]
fn custom_destination() {
    let result = nya::run(vec![
        create_middleware(|files: &mut Vec<SimpleFile>| {
            let file = &mut files[0];
            file.content = "a third test".to_string();
        }),
    ], Some("fixtures/custom_destination"), Some("_site_2"));
    if let Ok(r) = result {
        assert_eq!(r[0].content, "a third test".to_string());
        let path = Path::new("_site_2/mycoolfile.txt");
        assert_eq!(path.exists(), true);
    }
}
