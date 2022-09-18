mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let x = chunk_type::ChunkType::try_from([97,42,34,200]).unwrap();
    println!("{:#?}",x.bytes());
    todo!();
}
