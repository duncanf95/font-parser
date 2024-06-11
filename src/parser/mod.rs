pub mod tables;

use std::fs::File;
use std::io::Read;

pub fn read_file(file_name: &str) -> Vec<u8> {
    let mut file = File::open(file_name).expect("could not find file");
    let mut buffer: Vec<u8> = Vec::new();

    file.read(&mut buffer).expect("could not read file");
    file.read_to_end(&mut buffer).unwrap();

    buffer
}
