use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};

pub fn read_uint32_le_from_file(file_path: &str, offset: u64) -> io::Result<u32> {
    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::Start(offset))?;
    
    let mut buffer = [0; 4]; // 4 bytes for a UInt32
    file.read_exact(&mut buffer)?;
    let mut cursor = io::Cursor::new(buffer);
    let num = cursor.read_u32::<LittleEndian>()?;
    Ok(num)
}

pub fn buffer_alloc(size: usize, fill: u8) -> Vec<u8> {
    vec![fill; size]
}