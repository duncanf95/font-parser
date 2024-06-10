use crate::parser::read_file;
use crate::parser::tables::TableDirectory;

mod parser;

pub fn parse(font_file: &str) {
    let mut bytes = read_file(font_file);
    let table_directory = TableDirectory::new(&mut bytes);

    println!("test");
}


