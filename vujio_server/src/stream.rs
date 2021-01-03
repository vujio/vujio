use matroska::Matroska;
use std::fs::File;

pub use matroska::*;

pub fn unwrap_stream_data() {
    let f = File::open("filename.mkv").unwrap();
    let matroska = Matroska::open(f).unwrap();
    println!("title : {:?}", matroska.info.title);
}
