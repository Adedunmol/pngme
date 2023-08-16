use crate::{Error, Result};
use std::{str::FromStr, fmt};

#[derive(Debug, PartialEq)]
pub struct ChunkType {
    chunk_type: [u8; 4], // Specifies the type of the chunk in a png file and it is not more than 4 bytes
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        if value.len() != 4 {
            return Err("ChunkType only accepts an array of 4 elements".into())
        }
        
        let lower_range = 65..91; // This range represents the ASCII codes to upper case A - Z
        let upper_range = 97..123; // This range represents the ASCII codes to lower case a - z

        for byte in &value {
            if !lower_range.contains(byte) && !upper_range.contains(byte) {
                return Err("ChunkType only accepts bytes with the range(65 - 90) and (97 - 122)".into())
            }
        }

        Ok(ChunkType{ chunk_type: value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err("ChunkType can only take 4 characters".into())
        }

        if !s.is_ascii() {
            return Err("ChunkType takes ASCII characters only".into())
        }

        let lower_range = 65..91;
        let upper_range = 97..123;

        for byte in s.as_bytes() {
            if !lower_range.contains(byte) && !upper_range.contains(byte) {
                return Err("ChunkType only accepts bytes with the range(65 - 90) and (97 - 122)".into())
            }
        }

        let chunk_type: [u8; 4] = s.as_bytes().try_into()?;

        Ok(ChunkType { chunk_type })
    }

}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {

        self.chunk_type
    }

    /// This checks if a type represents a critical chunk(cannot be ignored be ignored by the decoder)
    /// or an ancillary chunk(can be ignored by the decoder)
    pub fn is_critical(&self) -> bool {

        self.chunk_type[0].is_ascii_uppercase()
    }

    pub fn to_string(&self) -> String {

        std::str::from_utf8(&self.chunk_type).expect("Invalid utf-8").to_string()
    }

    /// If the second byte is an uppercase ASCII letter, then the type is public
    /// otherwise it is private
    pub fn is_public(&self) -> bool {

        self.chunk_type[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {

        self.chunk_type[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {

        self.chunk_type[3].is_ascii_lowercase()
    }

    pub fn is_valid(&self) -> bool {

        let mut result: Vec<bool> = vec![];

        let lower_range = 67..91;
        let upper_range = 97..123;

        for byte in &self.chunk_type {
            if !lower_range.contains(byte) && !upper_range.contains(byte) {
                result.push(false)
            }
        }

        let invalid_byte = result.into_iter().any(|val| val == false);

        self.is_reserved_bit_valid() && !invalid_byte
    }
}

impl fmt::Display for ChunkType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "{:?}", self.chunk_type)
    }
}

// #[allow(unused_variables)]
// fn main() {
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
// }
