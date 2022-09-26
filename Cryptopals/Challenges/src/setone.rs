use crate::conversions::Convert;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Challenges;


//cryptopals rule: Always operate on raw bytes, not on enconded strings

impl Challenges {

    //used for single byte XOR cipher. Each letter that matches here will increase the weight of that string as being the correctly decyphered string
    //may be better to use hashmap here so we don't need to iterate through an array for each character.
    const VOWELS: [char; 12] = [ 'a','e','i','o','u','y',
                                 'A','E','I','O','U','Y' ];

    //used to decrypt a hex string that has been XOR'd against a single character to reveal a hidden message..
    //this takes a hex string, converts it to u8 hex vector, converts that to a u8 decimal vector, XOR's the result
    //from characters 32 to 126 and finds the most likely decrypted string based on character frequency
    pub fn singlebyte_xor_cipher(string: &str) -> String{
        let bytes = string.to_lowercase().as_bytes().to_vec();
        let hex = Convert::hex_to_dec(Convert::string_to_hex(bytes));
        let mut variations: Vec<String> = Vec::new();
        let mut highest = 0;
        let mut position = 0;
        for i in 32..127 {
            let mut temp: Vec<u8> = Vec::new();
            let mut count = 0;
            for j in  0..hex.len() {
                //XORs the decimal value if it's a valid character
                if !((hex[j] ^ i) > 126){
                    temp.push(hex[j] ^ i);
                }
                else {
                    temp.push(126);
                }
            }
            //here we get a tally to determine which is the decrypted string based on vowel count
            for j in 0..temp.len() {
                for f in 0..Challenges::VOWELS.len() {
                    if temp[j] == Challenges::VOWELS[f] as u8 {
                        count = count + 1;
                    }
                }
                //and take away a tally if it has characters not commonly used in sentences.
                if (temp[j] >= 91 && temp[j] <= 96) || (temp[j] >= 58 && temp[j] <= 64) || (temp[j] >= 33 && temp[j] <=  47) || (temp[j] >= 123 && temp[j] <=  127) || temp[j] == b'\n' {
                    count = count - 1;
                }
            }
            if highest < count {
                highest = count;
                position = i-32;
            }
            variations.push(std::string::String::from_utf8(temp).unwrap());
        }
        variations.remove(position.into())

    }

    //Xors two hex strings together and returns the product
    pub fn fixed_xor(string1: &str, string2: &str) -> String {
        if string1.len() != string2.len() {
            panic!("The strings must be equal length!");
        }
        let hex1 = Convert::string_to_hex(string1.to_lowercase().as_bytes().to_vec());
        let hex2 = Convert::string_to_hex(string2.to_lowercase().as_bytes().to_vec());
        let mut xored: Vec<u8> = Vec::new();
        let mut ans = String::new();
        for i in 0..hex1.len() {
            xored.push(hex1[i] ^ hex2[i]);
        }
        let converted = Convert::hex_to_string(xored);
        ans.push_str(std::str::from_utf8(&converted).unwrap());
        ans
    }

    //takes a file location of a list of new line seperated hex strings that have been single byte XOR ciphered
    //and finds the message.
    pub fn detect_xor(string: &str) -> String{
        let mut variations: Vec<String> = Vec::new();
        let content = File::open(string).expect("Unable to open file");
        let content = BufReader::new(content);
        let mut highest = 0;
        let mut position = 0;
        //adds to a vector all of the highest likely deciphered strings for each hex string
        for line in content.lines() {
            let dec = Challenges::singlebyte_xor_cipher(&line.as_ref().unwrap());
            variations.push(dec);
        }
        //finds the highest likely deciphered string amongst all the deciphered strings.
        for i in 0..variations.len() {
            let mut count = 0;
            for j in 0..variations[i].len() {
                let temp = variations[i].chars().nth(j).unwrap() as u8;
                //here we get a tally to determine which is the decrypted string based on vowel count
                for f in 0..Challenges::VOWELS.len() {
                    if temp == Challenges::VOWELS[f] as u8 || temp == 32{
                        count = count + 1;
                    }
                }
                //and take away a tally if it has characters not commonly used in sentences.
                if (temp >= 91 && temp <= 96) || (temp >= 58 && temp <= 64) || (temp >= 32 && temp <=  47) || (temp >= 123 && temp <=  127) {
                    count = count - 1;
                }
            }
            if highest < count {
                highest = count;
                position = i;
            }
        }
        variations.remove(position)
    }

    pub fn repeatkey_xor(string: &str, repeater: &str) -> String {
        let mut repeat_index = 0;
        let mut bytes: Vec<u8> = Vec::new();
        for i in 0..string.len() {
            let byte = (string.chars().nth(i).unwrap() as u8) ^ (repeater.chars().nth(repeat_index).unwrap() as u8);
            bytes.push(byte);

            repeat_index = repeat_index + 1;
            if repeat_index > repeater.len() - 1 {
                repeat_index = 0;
            }
        }
        std::string::String::from_utf8(Convert::hex_to_string(Convert::dec_to_hex(bytes))).unwrap()
    }
}