use std::str::from_utf8;
use crate::data_stream::DataStream;

pub struct TableDirectory {
    sfnt_version: u32,
    num_tables: u16,
    table_records: Vec<TableRecord>
}

impl TableDirectory {
    pub fn new(mut data_stream: &mut DataStream) -> Self {
        let sfnt_version = data_stream.read::<u32>().unwrap();
        let num_tables = data_stream.read::<u16>().unwrap();
        data_stream.advance(6);
        let mut table_records: Vec<TableRecord> = Vec::new();

        for i in 0..num_tables {
            let table_record = TableRecord::new(&mut data_stream);
            table_records.push(table_record);
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
                b"avar" => println!("table is avar"),
                b"BASE" => println!("table is BASE"),
                b"CBDT" => println!("table is CBDT"),
                b"CBLC" => println!("table is CBLC"),
                b"CFF " => println!("table is CFF"),
                b"CFF2" => println!("table is CFF2"),
                b"cmap" => println!("table is cmap"),
                b"COLR" => println!("table is COLR"),
                b"CPAL" => println!("table is CPAL"),
                b"cvar" => println!("table is cvar"),
                b"cvt " => println!("table is cvt"),
                b"DSIG" => println!("table is DSIG"),
                b"EBDT" => println!("table is EBDT"),
                b"EBLC" => println!("table is EBLC"),
                b"EBSC" => println!("table is EBSC"),
                b"fpgm" => println!("table is fpgm"),
                b"fvar" => println!("table is fvar"),
                b"gasp" => println!("table is gasp"),
                b"GDEF" => println!("table is GDEF"),
                b"glyf" => println!("table is glyf"),
                b"GPOS" => println!("table is GPOS"),
                b"GSUB" => println!("table is GSUB"),
                b"gvar" => println!("table is gvar"),
                b"hdmx" => println!("table is hdmx"),
                b"head" => println!("table is head"),
                b"hhea" => println!("table is hhea"),
                b"hmtx" => println!("table is hmtx"),
                b"HVAR" => println!("table is HVAR"),
                b"JSTF" => println!("table is JSTF"),
                b"kern" => println!("table is kern"),
                b"loca" => println!("table is loca"),
                b"LTSH" => println!("table is LTSH"),
                b"MATH" => println!("table is MATH"),
                b"maxp" => println!("table is maxp"),
                b"MERG" => println!("table is MERG"),
                b"meta" => println!("table is meta"),
                b"MVAR" => println!("table is MVAR"),
                b"PCLT" => println!("table is PCLT"),
                b"post" => println!("table is post"),
                b"prep" => println!("table is prep"),
                b"sbix" => println!("table is sbix"),
                b"STAT" => println!("table is STAT"),
                b"SVG " => println!("table is SVG"),
                b"VDMX" => println!("table is VDMX"),
                b"vhea" => println!("table is vhea"),
                b"vmtx" => println!("table is vmtx"),
                b"VORG" => println!("table is VORG"),
                b"VVAR" => println!("table is VVAR"),
                b"name" => println!("table is name"),
                b"OS/2" => println!("table is OS/2"),
                _=> println!("Unknown Table! {:?}", from_utf8(&table_record.tag))
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

