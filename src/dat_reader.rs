use std::io::Read;
use byteorder::{LittleEndian, ByteOrder};

/// dat 头结构
#[repr(align(1))]
pub struct Header {
    // 2 bytes
    total_len: u16,
    // 4 bytes
    r#type: u32,
    // 2 bytes
    data_len: u16,
} // 8 bytes

fn buf_reader<T: Read, const size: usize>(reader: &mut T) -> [u8; size]{
    let mut buf = [0; size];
    reader.read(&mut buf[..]);
    buf
}

impl Header {
    fn new<T: Read>(reader: &mut T) -> Header {
        Self {
            total_len: LittleEndian::read_u16(&buf_reader::<T, 2>(reader)),
            r#type: LittleEndian::read_u32(&buf_reader::<T, 4>(reader)),
            data_len: LittleEndian::read_u16(&buf_reader::<T, 2>(reader)),
        }
    }
}

pub struct DatReader {

}
