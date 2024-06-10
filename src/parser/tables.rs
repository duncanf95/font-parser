use std::mem::size_of;
use crate::parser::ByteConverter;

pub struct TableDirectory {
    sfnt_version: u32,
    num_tables: u16,
    table_records: Vec<TableRecord>
}

impl TableDirectory {
    pub fn new(font_data: &mut Vec<u8>) -> Self {
        let (sfnt_version, mut font_data) = ByteConverter::get_next_u32(font_data);
        let (num_tables, mut font_data) = ByteConverter::get_next_u16(&mut font_data);
        let mut table_records: Vec<TableRecord> = Vec::new();

        for i in 0..num_tables {
            let (table_record, mut new_font_data) = TableRecord::new(&mut font_data);
            table_records.push(table_record);
            font_data = new_font_data;
        }

        Self {
            sfnt_version,
            num_tables,
            table_records
        }
    }
}

pub struct TableRecord {
    tag: [u8;4],
    check_sum: u32,
    offset: u32,
    length: u32,
}

impl TableRecord {
    pub fn new(font_data: &mut Vec<u8>) -> (Self, Vec<u8>) {
        let (tag, mut font_data) = Self::get_tag(font_data);
        let (check_sum, mut font_data) = ByteConverter::get_next_u32(&mut font_data);
        let (offset, mut font_data) = ByteConverter::get_next_u32(&mut font_data);
        let (length, mut font_data) = ByteConverter::get_next_u32(&mut font_data);

        let table_record = Self {
            tag,
            check_sum,
            offset,
            length
        };

        (table_record, font_data)
    }

    fn get_tag(font_data: &mut Vec<u8>) -> ([u8;4], Vec<u8>) {
        let (tag_bytes, font_data) = font_data.split_at_mut(4);
        let tag = ByteConverter::to_raw_u32(tag_bytes.to_vec());
        (tag, font_data.to_vec())
    }
}

