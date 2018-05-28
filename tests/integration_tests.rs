extern crate seven;

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
