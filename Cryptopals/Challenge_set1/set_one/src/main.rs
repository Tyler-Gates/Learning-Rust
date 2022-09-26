use crate::setone::Challenges;
use crate::conversions::Convert;

mod setone;
mod conversions; 

fn main() {
    //challenge 1
    println!("Answer for Challenge 1: {}", Convert::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
    //challenge 2
    println!("Answer for Challenge 2: {}", Challenges::fixed_xor("1c0111001f010100061a024b53535009181c","686974207468652062756c6c277320657965"));
    //challenge 3
    println!("Answer for Challenge 3: {}", Challenges::singlebyte_xor_cipher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
    //challenge 4
    println!("Answer for Challenge 4: {}", Challenges::detect_xor("./src/4.txt"));
    //challenge 5
    println!("Answer for Challenge 5: {}", Challenges::repeatkey_xor("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", "ICE"));
    //challenge 6
}