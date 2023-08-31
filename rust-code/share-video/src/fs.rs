use std::{fs::File, io::Read};

pub fn read_index() -> String {
    let mut file = File::open("/Users/sanmws/chimhong/coding/rust-code/share-video/src/index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
