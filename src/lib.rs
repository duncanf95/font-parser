use crate::data_stream::DataStream;
use crate::parser::read_file;
use crate::parser::tables::TableDirectory;

mod parser;
mod data_stream;

pub fn parse(font_file: &str) {
    let bytes = read_file(font_file);
    let mut data_stream = DataStream::new(&bytes);
    let mut table_directory = TableDirectory::new(&mut data_stream);

    table_directory.format_tables();
    println!("test")
}


