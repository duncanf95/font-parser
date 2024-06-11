use std::mem::size_of;

pub struct DataStream<'a> {
    bytes: &'a [u8],
    offset: usize
}

impl<'a> DataStream<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            offset: 0
        }
    }
    pub fn read<Data: FromBytes>(&mut self) -> Option<Data> {
        self.read_bytes(size_of::<Data>()).and_then(Data::parse)
    }

    pub fn read_bytes(&mut self, length: usize) -> Option<&'a [u8]> {
        let bytes = &self.bytes[self.offset..self.offset + length];
        self.advance(length);
        Some(bytes)
    }

    pub fn read_bytes_at(&self, offset: usize, length: usize) -> Option<&'a [u8]> {
        let bytes = &self.bytes[offset..offset + length];
        Some(bytes)
    }

    pub fn advance(&mut self, length: usize) {
        self.offset += length;
    }

    pub fn reset_offset(&mut self) {
        self.offset = 0;
    }
}

pub trait FromBytes {
    fn parse(data: &[u8]) -> Option<Self> where Self: Sized;
}

impl FromBytes for u32 {
    fn parse(data: &[u8]) -> Option<Self> {
        data.try_into().ok().map(u32::from_be_bytes)
    }
}

impl FromBytes for u16 {
    fn parse(data: &[u8]) -> Option<Self> {
        data.try_into().ok().map(u16::from_be_bytes)
    }
}

impl FromBytes for [u8;4] {
    fn parse(data: &[u8]) -> Option<Self> {
        Some(data.try_into().unwrap())
    }
}