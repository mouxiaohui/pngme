use core::fmt;

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidByte(u8),
    InvalidLength,
}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkTypeError::InvalidByte(byte) => write!(f, "Invalid byte: {}", byte),
            ChunkTypeError::InvalidLength => {
                write!(f, "Chunk types must be 4 characters (bytes) long")
            }
        }
    }
}

impl std::error::Error for ChunkTypeError {}

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// 块类型，第一个字节的5位，0-关键数据块，1-辅助数据块
    pub fn is_critical(&self) -> bool {
        self.bytes[0] & 32 == 0
    }

    /// 块类型，第二个字节的5位，0-公开，1-私有
    pub fn is_public(&self) -> bool {
        self.bytes[1] & 32 == 0
    }

    /// 块类型，第三个字节的5位，必须为0(大写字母)
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2] & 32 == 0
    }

    /// 块类型，第四个字节的5位，0-复制不安全，1-复制安全
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3] & 32 == 32
    }

    /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        for b in self.bytes.iter() {
            if !b.is_ascii_lowercase() && !b.is_ascii_uppercase() {
                return false;
            }
        }

        return self.is_reserved_bit_valid();
    }
}

impl std::convert::TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for b in value.iter() {
            if !b.is_ascii_lowercase() && !b.is_ascii_uppercase() {
                return Err(ChunkTypeError::InvalidByte(*b));
            }
        }

        Ok(Self { bytes: value })
    }
}

impl std::str::FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(ChunkTypeError::InvalidLength);
        }

        let sb = s.as_bytes();
        for b in sb {
            if !b.is_ascii_lowercase() && !b.is_ascii_uppercase() {
                return Err(ChunkTypeError::InvalidByte(*b));
            }
        }

        Ok(Self {
            bytes: [sb[0], sb[1], sb[2], sb[3]],
        })
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&self.bytes).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{convert::TryFrom, str::FromStr};

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
