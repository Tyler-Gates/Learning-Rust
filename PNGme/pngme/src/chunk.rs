use crc::{Crc, Algorithm};
use crate::chunk_type::{ChunkTypeError, ChunkType};

const CUSTOM_ALG: Algorithm<u32> = Algorithm {
    poly: 0x04c11db7,
    init: 0xffffffff,
    refin: true,
    refout: true,
    xorout: 0xffffffff,
    check: 0xffffffff,
    residue: 0x00000000
};

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len();
        let crc_worker = Crc::<u32>::new(&CUSTOM_ALG);
        let mut data_and_type: Vec<u8> = Vec::new();
        data_and_type.extend_from_slice(&chunk_type.bytes().to_vec());
        data_and_type.extend_from_slice(&data);
        let crc = Crc::<u32>::checksum(&crc_worker, &data_and_type);
        Chunk { length: length.try_into().unwrap(), chunk_type, chunk_data: data, crc }
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    pub fn data_as_string(&self) -> Result<String, ChunkTypeError> {
        if std::str::from_utf8(&self.chunk_data).is_err() {
            return Err(ChunkTypeError::InvalidArgs { details: "Not utf8!".to_string() } );
        }
        Ok(std::str::from_utf8(&self.chunk_data).unwrap().to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.chunk_type.bytes());
        for i in 0..self.chunk_data.len() {
            bytes.push(self.chunk_data[i]);
        }
        bytes.extend_from_slice(&self.crc.to_be_bytes());
        bytes
    }
}

impl std::convert::TryFrom<&[u8]> for Chunk {
    type Error = ChunkTypeError;

    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {


        let mut length: Vec<u8> = Vec::new();
        for i in 0..4 {
            length.push(input[i]);
        }
        let length = u32::from_be_bytes(length.try_into().unwrap());
        let mut chunk_type: Vec<u8> = Vec::new();


        for i in 4..8 {
            chunk_type.push(input[i]);
        }
        let chunk_type: [u8;4] = chunk_type.try_into().unwrap();
        let mut chunk_type = ChunkType::try_from(chunk_type).unwrap();


        let mut chunk_data: Vec<u8> = Vec::new();
        for i in 8..input.len()-4 {
            chunk_data.push(input[i]);
        }


        let mut crc: Vec<u8> = Vec::new();
        for i in input.len()-4..input.len() {
            crc.push(input[i]);
        }
        let crc = u32::from_be_bytes(crc.try_into().unwrap());


        //compare entered crc to computed
        let crc_worker = Crc::<u32>::new(&CUSTOM_ALG);
        let mut data_and_type: Vec<u8> = Vec::new();
        data_and_type.extend_from_slice(&chunk_type.bytes().to_vec());
        data_and_type.extend_from_slice(&chunk_data);
        let computed = Crc::<u32>::checksum(&crc_worker, &data_and_type);
        if crc != computed {
            return Err(ChunkTypeError::InvalidArgs { details: "Incorrect Checksum!".to_string() } );
        }

        Ok(Chunk { length, chunk_type, chunk_data, crc})
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "length:{} \nchunk_type string:{} \nchunk_data bytes:{:?} \nchunk_data string: {}\ncrc:{}", self.length(), self.chunk_type.to_string(), self.data(), self.data_as_string().unwrap(), self.crc())
    }
}


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
