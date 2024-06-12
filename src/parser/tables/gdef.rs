use crate::data_stream::DataStream;
use crate::parser::tables::{ClassDefinitionFormatOneHeader, ClassDefinitionFormatTwoHeader};

pub struct GDEFHeader {
    version: String,
    glyph_class_def_offset: u16,
    attach_list_offset: u16,
    lig_caret_list_offset: u16,
    mark_attach_class_def_offset: u16,
    mark_glyph_sets_def_offset: Option<u16>,
    item_var_store_offset: Option<u16>
}

impl GDEFHeader {
    pub fn new(data_stream: &mut DataStream) -> Self {
        let major_version = data_stream.read::<u16>().unwrap();
        let minor_version = data_stream.read::<u16>().unwrap();

        let glyph_class_def_offset = data_stream.read::<u16>().unwrap();
        let attach_list_offset = data_stream.read::<u16>().unwrap();
        let lig_caret_list_offset = data_stream.read::<u16>().unwrap();
        let mark_attach_class_def_offset = data_stream.read::<u16>().unwrap();
        let mut mark_glyph_sets_def_offset = None;
        let mut item_var_store_offset = None;
        if minor_version >= 2 {
            mark_glyph_sets_def_offset = data_stream.read::<u16>();
            if minor_version >= 3 {
                item_var_store_offset = data_stream.read::<u16>();
            }
        }

        let version = major_version.to_string() + "." + minor_version.to_string().as_str();

        Self {
            version,
            glyph_class_def_offset,
            attach_list_offset,
            lig_caret_list_offset,
            mark_attach_class_def_offset,
            mark_glyph_sets_def_offset,
            item_var_store_offset,
        }
    }
}

pub struct GDEFTable {
    header: GDEFHeader,
    glyph_class_definition_table: GlyphClassDefinitionTable
}

impl GDEFTable {
    pub fn new(data_stream: &mut DataStream) -> Self {
        let header = GDEFHeader::new(data_stream);
        let glyph_class_definition_table = GlyphClassDefinitionTable::new(data_stream, header.glyph_class_def_offset);


        Self {
            header,
            glyph_class_definition_table
        }
    }
}

pub struct GlyphClassDefinitionTable {
    format: u16,
    format_one: Option<ClassDefinitionFormatOneHeader>,
    format_two: Option<ClassDefinitionFormatTwoHeader>
}

impl GlyphClassDefinitionTable {
    pub fn new(data_stream: &mut DataStream, offset: u16) -> Self {
        data_stream.reset_offset();
        data_stream.advance(offset as usize);
        let format = data_stream.read::<u16>().unwrap();
        let mut format_one = None;
        let mut format_two = None;
        match &format {
            1 => format_one = Some(ClassDefinitionFormatOneHeader::new(data_stream)),
            2 => format_two = Some(ClassDefinitionFormatTwoHeader::new(data_stream)),
            _=> println!("INVALID FORMAT")
        }

        Self {
            format,
            format_one,
            format_two,
        }
    }
}

