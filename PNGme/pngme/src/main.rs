mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::str::FromStr;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let x = chunk_type::ChunkType::try_from([82, 117, 83, 116]).unwrap();
    let y = chunk_type::ChunkType::from_str("Rust").unwrap();
    println!("try_from:{:#?}",x.bytes());
    println!("is_critical:{:#?}",x.is_critical());
    println!("try_from:{:#?}",x.bytes());

    println!("from_str:{:#?}",y.bytes());
    println!("is_critical:{:#?}",y.is_critical());
    println!("from_str:{:#?}",y.bytes());
    Ok(())
}
