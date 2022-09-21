use std::fs;
use crate::chunk_type::ChunkType;
use std::str::FromStr;
use crate::chunk::Chunk;
use crate::png::Png;

pub struct Commands;

impl Commands {
    pub fn encode(path: String, chunk_type_string: String, message_string: String) {

        let chunk_type = ChunkType::from_str(&chunk_type_string);
        let message = message_string.clone().into_bytes();
        let chunk = Chunk::new(chunk_type.unwrap(),message);
        let mut file: Vec<u8> = fs::read(&path).unwrap();
        let mut png = Png::try_from(&file[..]).unwrap();
        png.append_chunk(chunk);
        fs::write(&path, png.as_bytes()).expect("Unable to write to file");
        println!("Appended message {} with chunk_type {} in {}", message_string, chunk_type_string, path);
    }
    pub fn decode(path: String, chunk_type_string: String) {
        //chunk creation
        let mut file: Vec<u8> = std::fs::read(&path).unwrap();
        let mut png = Png::try_from(&file[..]).unwrap();
        let decode = png.chunk_by_type(&chunk_type_string).unwrap();
        println!("{}", decode);
    }

    pub fn remove(path: String, chunk_type_string: String) {
        let mut file: Vec<u8> = std::fs::read(&path).unwrap();
        let mut png = Png::try_from(&file[..]).unwrap();
        png.remove_chunk(&chunk_type_string);
    }

    pub fn print(path: String) {
        let mut file: Vec<u8> = std::fs::read(&path).unwrap();
        let mut png = Png::try_from(&file[..]).unwrap();
        println!("{}", png);
    }
}