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
        font_data = font_data.split_off(6);
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

    pub fn match_tables(&self) {
        for table_record in &self.table_records {
            match &table_record.tag {
                b"bdat" => println!("table is bdat"),
                b"bloc" => println!("table is bloc"),
                b"CFF " => println!("table is CFF"),
                b"CBDT" => println!("table is CBDT"),
                b"CBLC" => println!("table is CBLC"),
                b"CFF2" => println!("table is CFF2"),
                b"COLR" => println!("table is COLR"),
                b"CPAL" => println!("table is CPAL"),
                b"EBDT" => println!("table is EBDT"),
                b"EBLC" => println!("table is EBLC"),
                b"GDEF" => println!("table is GDEF"),
                b"GPOS" => println!("table is GPOS"),
                b"GSUB" => println!("table is GSUB"),
                b"MATH" => println!("table is MATH"),
                b"HVAR" => println!("table is HVAR"),
                b"MVAR" => println!("table is MVAR"),
                b"OS/2" => println!("table is OS/2"),
                b"SVG " => println!("table is SVG "),
                b"VORG" => println!("table is VORG"),
                b"VVAR" => println!("table is VVAR"),
                b"ankr" => println!("table is ankr"),
                b"avar" => println!("table is avar"),
                b"cmap" => println!("table is cmap"),
                b"feat" => println!("table is feat"),
                b"fvar" => println!("table is fvar"),
                b"glyf" => println!("table is glyf"),
                b"gvar" => println!("table is gvar"),
                b"head" => println!("table is head"),
                b"hhea" => println!("table is hhea"),
                b"hmtx" => println!("table is hmtx"),
                b"kern" => println!("table is kern"),
                b"kerx" => println!("table is kerx"),
                b"loca" => println!("table is loca"),
                b"maxp" => println!("table is maxp"),
                b"morx" => println!("table is morx"),
                b"name" => println!("table is name"),
                b"post" => println!("table is post"),
                b"sbix" => println!("table is sbix"),
                b"trak" => println!("table is trak"),
                b"vhea" => println!("table is vhea"),
                b"vmtx" => println!("table is vmtx"),
                _=> println!("Unknown Table!")
            }
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

