use crate::data_stream::DataStream;

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