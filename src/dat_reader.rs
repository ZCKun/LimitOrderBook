use std::io::{Read, BufRead, BufReader};
use byteorder::{LittleEndian, ByteOrder};
use std::fs::{File, OpenOptions};

/// dat 头结构
#[derive(Debug)]
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
    pub(crate) fn new<T: Read>(reader: &mut T) -> Header {
        Self {
            total_len: LittleEndian::read_u16(&buf_reader::<T, 2>(reader)),
            r#type: LittleEndian::read_u32(&buf_reader::<T, 4>(reader)),
            data_len: LittleEndian::read_u16(&buf_reader::<T, 2>(reader)),
        }
    }
}

pub struct DatReader {
    buf_reader: BufReader<File>
}

impl DatReader {
    pub(crate) fn new(file_path: &str) -> DatReader {
        let file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .expect(format!("can't open file {}", file_path).as_str());
        Self {
            buf_reader: BufReader::new(file)
        }
    }

    pub fn read(&mut self){
        while !self.buf_reader.fill_buf()?.is_empty() {
            let header = Header::new(&mut self.buf_reader);
            let data = buf_reader::<BufReader<File>, DATA_LEN>(&mut self.buf_reader);
            println!("{:?}", data[..8]);
        }
    }
}
