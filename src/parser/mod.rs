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

pub struct ByteConverter;

impl ByteConverter {

    pub fn get_next_u32(mut font_data: &mut Vec<u8>) -> (u32, Vec<u8>) {
        let (u32_bytes, font_data) = font_data.split_at_mut(4);
        let u32_raw = Self::to_raw_u32(u32_bytes.to_vec());
        (u32::from_be_bytes(u32_raw), font_data.to_vec())
    }

    pub fn get_next_u16(font_data: &mut Vec<u8>) -> (u16, Vec<u8>) {
        let (u16_bytes, font_data) = font_data.split_at_mut(2);
        let u16_raw = Self::to_raw_u16(u16_bytes.to_vec());
        (u16::from_be_bytes(u16_raw), font_data.to_vec())
    }
    fn to_raw_u32(data: Vec<u8>) -> [u8;4] {
        data.try_into().expect("incorrect slice length for u32")
    }

    fn to_raw_u16(data: Vec<u8>) -> [u8;2] {
        data.try_into().expect("incorrect slice length for u16")
    }
}
