use std::fmt;

use crate::{Error, Result, chunk_type::ChunkType};
use crc::{Crc, CRC_32_ISO_HDLC};

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: [u8; 4],
    chunk_data: Vec<u8>,
    crc: u32 // (Cyclic Redundancy Check)
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self> {

        pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

        // The first 4 bytes represent the length
        let length_bytes: [u8; 4] = value[..4].try_into().unwrap();
        let length = u32::from_be_bytes(length_bytes);

        // The next 4 bytes represent the chunk_type
        let chunk_type: [u8; 4] = value[4..8].try_into().unwrap();

        // The next bytes of length "length" represent the data
        let end = 8 + length;
        let chunk_data: Vec<u8> = value[8..end as usize].try_into().unwrap();

        // The remaining bytes are for the crc
        let chunk_length = value.len();
        let start = chunk_length - 4;
        let crc_bytes: [u8; 4] = value[start..].try_into().unwrap();
        let crc = u32::from_be_bytes(crc_bytes);

        let correct_crc = CASTAGNOLI.checksum(&value[4..end as usize]);

        if crc != correct_crc {
            return Err("Invalid crc (Cyclic Redundancy Check)".into())
        }

        Ok( Chunk { length, chunk_type, chunk_data, crc } )
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {

        pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        
        let length: u32 = data.len().try_into().unwrap();
        let new_chunk_data = &data[..];

        let new_data = [&chunk_type.bytes()[..], new_chunk_data].concat();

        let crc = CASTAGNOLI.checksum(&new_data[..]);

        Chunk { length, chunk_type: chunk_type.bytes(), chunk_data: data, crc }
    }

    pub fn length(&self) -> u32 {

        self.length
    }

    pub fn crc(&self) -> u32 {

        self.crc
    }

    pub fn chunk_type(&self) -> ChunkType {

        ChunkType::try_from(self.chunk_type).unwrap()
    }

    pub fn data_as_string(&self) -> Result<String> {

        let data = std::str::from_utf8(&self.chunk_data).expect("Invalid UTF-8").to_string();

        Ok(data)
    }

    pub fn as_bytes(&self) -> Vec<u8> {

        let result = self.length
                            .to_be_bytes()
                            .iter()
                            .cloned()
                            .chain(self.chunk_type.iter().cloned())
                            .chain(self.chunk_data.iter().cloned())
                            .chain(self.crc.to_be_bytes().iter().cloned())
                            .collect();

        result
    }
}

impl fmt::Display for Chunk {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk {{").unwrap();
        write!(f, " Length: {}", self.length).unwrap();
        write!(f, " crc: {}", self.crc).unwrap();
        write!(f, " data: {:?}", self.chunk_data).unwrap();
        write!(f, " type: {:?}", self.chunk_type).unwrap();
        write!(f, "}}").unwrap();

        Ok(())
    }
}

// #![allow(unused_variables)]
// fn main() {
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
// }