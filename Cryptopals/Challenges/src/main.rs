use crate::setone::SetOne;
use crate::conversions::Convert;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod setone;
mod conversions; 

fn main() {
    //Set One
    s1_challenge_one("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    s1_challenge_two("1c0111001f010100061a024b53535009181c","686974207468652062756c6c277320657965");
    s1_challenge_three("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    s1_challenge_four("./src/4.txt");
    s1_challenge_five("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", "ICE");
    s1_challenge_six("./src/6.txt");
    //challenge_seven("./src/7.txt");
}

fn get_from_file(path: &str) -> String{
    let content = File::open(path).expect("Unable to open file");
    let content = BufReader::new(content);
    let mut string = String::new();
    for line in content.lines() {
        let line = line.unwrap();
        string.push_str(&line[0..line.len()]);
    }
    string
}

fn s1_challenge_one(hex: &str) {
    println!("Answer for Challenge 1: {}",Convert::hex_to_base64(hex));
}

fn s1_challenge_two(hex1: &str, hex2: &str) {
    println!("Answer for Challenge 2: {}",SetOne::fixed_xor(hex1,hex2));
}

fn s1_challenge_three(hex: &str) {
    let bytes = hex.to_lowercase().as_bytes().to_vec();
    let dec = Convert::hex_to_dec(Convert::string_to_hex(bytes));
    let (ans, _) = SetOne::singlebyte_xor_cipher(&dec);
    println!("Answer for Challenge 3: {}", ans);
}

fn s1_challenge_four(file_path: &str) {
    println!("Answer for Challenge 4: {}", SetOne::detect_xor(file_path));
}

fn s1_challenge_five(string: &str, key: &str) {
    println!("Answer for Challenge 5: {}", SetOne::repeatkey_xor(string,key));
}

fn s1_challenge_six(file_path: &str) {
    let six = get_from_file(file_path);
    println!("Answer for Challenge 6: {}", SetOne::break_repeatkey_xor(&six));
}

fn s1_challenge_seven(file_path: &str) {
    let seven = get_from_file(file_path);
    println!("Answer for Challenge 7: {}", SetOne::ecb_decrypt(&seven, "YELLOW SUBMARINE"));
}