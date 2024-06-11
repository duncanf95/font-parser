mod gdef;

use std::collections::HashMap;
use std::str::from_utf8;
use crate::data_stream::DataStream;
use crate::parser::tables::gdef::GDEFHeader;

pub struct TableDirectory<'a> {
    sfnt_version: u32,
    num_tables: u16,
    data_stream: &'a DataStream<'a>,
    table_records: Vec<TableRecord>,
    table_data: HashMap<String, DataStream<'a>>
}

impl<'a> TableDirectory<'a> {
    pub fn new(mut data_stream: &'a mut DataStream) -> Self {
        let sfnt_version = data_stream.read::<u32>().unwrap();
        let num_tables = data_stream.read::<u16>().unwrap();
        data_stream.advance(6);
        let mut table_records: Vec<TableRecord> = Vec::new();

        for i in 0..num_tables {
            let table_record = TableRecord::new(&mut data_stream);
            table_records.push(table_record);
        }

        let table_data = Self::populate_table_data(&table_records, data_stream);

        Self {
            sfnt_version,
            num_tables,
            table_records,
            table_data,
            data_stream
        }
    }

    fn populate_table_data(table_records: &Vec<TableRecord>, data_stream: &'a DataStream) -> HashMap<String, DataStream<'a>> {
        let mut table_data = HashMap::new();
        for table_record in table_records {
            let tag_string = Self::match_table(&table_record.tag);
            let data = data_stream.read_bytes_at(table_record.offset as usize, table_record.length as usize).unwrap();
            table_data.insert(tag_string, DataStream::new(data));
        }
        table_data
    }

    pub fn format_tables(&mut self) {
        let gdef_data = self.table_data.get_mut("GDEF").unwrap();
        let gdef_header = GDEFHeader::new(gdef_data);
    }

    pub fn match_table(table_tag: &[u8;4]) -> String {
        match table_tag {
            b"avar" => return "avar".to_string(),
            b"BASE" => return "BASE".to_string(),
            b"CBDT" => return "CBDT".to_string(),
            b"CBLC" => return "CBLC".to_string(),
            b"CFF " => return "CFF".to_string(),
            b"CFF2" => return "CFF2".to_string(),
            b"cmap" => return "cmap".to_string(),
            b"COLR" => return "COLR".to_string(),
            b"CPAL" => return "CPAL".to_string(),
            b"cvar" => return "cvar".to_string(),
            b"cvt " => return "cvt".to_string(),
            b"DSIG" => return "DSIG".to_string(),
            b"EBDT" => return "EBDT".to_string(),
            b"EBLC" => return "EBLC".to_string(),
            b"EBSC" => return "EBSC".to_string(),
            b"fpgm" => return "fpgm".to_string(),
            b"fvar" => return "fvar".to_string(),
            b"gasp" => return "gasp".to_string(),
            b"GDEF" => return "GDEF".to_string(),
            b"glyf" => return "glyf".to_string(),
            b"GPOS" => return "GPOS".to_string(),
            b"GSUB" => return "GSUB".to_string(),
            b"gvar" => return "gvar".to_string(),
            b"hdmx" => return "hdmx".to_string(),
            b"head" => return "head".to_string(),
            b"hhea" => return "hhea".to_string(),
            b"hmtx" => return "hmtx".to_string(),
            b"HVAR" => return "HVAR".to_string(),
            b"JSTF" => return "JSTF".to_string(),
            b"kern" => return "kern".to_string(),
            b"loca" => return "loca".to_string(),
            b"LTSH" => return "LTSH".to_string(),
            b"MATH" => return "MATH".to_string(),
            b"maxp" => return "maxp".to_string(),
            b"MERG" => return "MERG".to_string(),
            b"meta" => return "meta".to_string(),
            b"MVAR" => return "MVAR".to_string(),
            b"PCLT" => return "PCLT".to_string(),
            b"post" => return "post".to_string(),
            b"prep" => return "prep".to_string(),
            b"sbix" => return "sbix".to_string(),
            b"STAT" => return "STAT".to_string(),
            b"SVG " => return "SVG".to_string(),
            b"VDMX" => return "VDMX".to_string(),
            b"vhea" => return "vhea".to_string(),
            b"vmtx" => return "vmtx".to_string(),
            b"VORG" => return "VORG".to_string(),
            b"VVAR" => return "VVAR".to_string(),
            b"name" => return "name".to_string(),
            b"OS/2" => return "OS/2".to_string(),
            _=> return "UNKNOWN".to_string()
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
    pub fn new(data_stream: &mut DataStream) -> Self {
        let tag = data_stream.read::<[u8;4]>().unwrap();
        let check_sum = data_stream.read::<u32>().unwrap();
        let offset = data_stream.read::<u32>().unwrap();
        let length = data_stream.read::<u32>().unwrap();

        let table_record = Self {
            tag,
            check_sum,
            offset,
            length
        };

        table_record
    }
}

