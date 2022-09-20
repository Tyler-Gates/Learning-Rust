
#[derive(Debug)]
pub struct ChunkType {
    bytes: [u8; 4],
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidArgs { details: String},
}


impl ChunkType {

    pub fn bytes(&self) -> [u8;4] {
        self.bytes
    }

    pub fn is_critical(&self) -> bool {
        self.bytes[0] >= 65 && self.bytes[0] <= 90
    }

    pub fn is_public(&self) -> bool {
        self.bytes[1] >= 65 && self.bytes[1] <= 90
    }
    
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2] >= 65 && self.bytes[2] <= 90
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3] >= 97 && self.bytes[3] <= 122
    }

    pub fn to_string(&self) -> String {
        std::str::from_utf8(&self.bytes).unwrap().to_string()
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.bytes() == other.bytes()
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        write!(f, "Byte1:{} Byte2:{} Byte3:{} Byte4:{}", self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3])
    }
}

impl std::str::FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() != 4{
            return Err(ChunkTypeError::InvalidArgs { details: "Must be 4 characters long!".to_string() } );
        }
        let mut bytes: [u8;4] = [0;4];
        for i in 0..4 {
            let temp = input.chars().nth(i).unwrap() as u8;
            if (temp <= 90 && temp >= 65) || (temp <= 122 && temp >= 97) {
                bytes[i] = temp;
            }   
            else{
                return Err(ChunkTypeError::InvalidArgs { details: "Alphabetical characters only!".to_string() } );
            }
        }
        Ok(ChunkType { bytes })
    }
}

impl std::convert::TryFrom<[u8;4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(input: [u8; 4]) -> Result<Self, Self::Error> {
        let mut bytes: [u8;4] = [0; 4];
        for i in 0..4 { 
            if (input[i] <= 90 && input[i] >= 65) || (input[i] <= 122 && input[i] >= 97) {
                bytes[i] = input[i];
            }   
            else{
                return Err(ChunkTypeError::InvalidArgs { details: "Alphabetical characters only!".to_string() } );
            }
        }

        Ok(ChunkType { bytes })
    }

}

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
