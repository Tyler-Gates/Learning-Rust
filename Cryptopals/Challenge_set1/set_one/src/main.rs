
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Convert;


//cryptopals rule: Always operate on raw bytes, not on enconded strings

impl Convert {

    //base64 table with each value indexed to it's binary value
    const BASE64_LOOKUP_TABLE: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                                             'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                                             'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                                             'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/' ];
    //base64 has padding characters when there aren't 3 bytes (24 bits) to work with. 
    const PADDING: char = '=';

    //used for single byte XOR cipher. Each letter that matches here will increase the weight of that string as being the correctly decyphered string
    //may be better to use hashmap here so we don't need to iterate through an array for each character.
    const VOWELS: [char; 12] = [ 'a','e','i','o','u','y',
                                 'A','E','I','O','U','Y' ];

    //Takes a string that is Hex encoded and converts it to a correct hex byte medium then is bit manipulated into base64.
    //Will panic is parameter string isn't a valid hexadecimal.
    pub fn hex_to_base64(string: &str) -> String {
        let bytes = string.to_lowercase().as_bytes().to_vec();
        let hex = Convert::string_to_hex(bytes);
        let mut ans = String::new();
        let mut i = 1;
        while i < hex.len() {
            //Grabs the first base64 byte by combining the first 4 of the first byte and last 2 of the second byte
            //Hexadecimal utilizes 4 bits, this example ignore the unused 4 bits. Each pair of *s are a new base 64 byte.
            //Base64 utilize 6 bits, leaving the last 2 as unused.
            // *0000 00*00 
            let byte = (hex[i-1] << 2) + (hex[i] >> 2);
            ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(byte)]);

            //if the end is reached, utilize the remaining bits,add padding to 24 bits, and break.
            if i+1 >= hex.len() {
                ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(hex[i] << 4)]);
                ans.push(Convert::PADDING);
                ans.push(Convert::PADDING);
                break;
            }

            // *0000 00*00 0000* now on the second base64 byte
            let prev = hex[i] << 6;
            let byte = (prev >> 2) + (hex[i+1]);
            ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(byte)]);

            if i+2 >= hex.len() {
                panic!("Must be Hexadecimal String! (two characters per hexadecimal value)")
            }

            //if the end is reached, utilize the remaining bits,add padding to 24 bits, and break.
            if i+3 >= hex.len() {
                ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(hex[i+2] << 2)]);
                ans.push(Convert::PADDING);
                break;
            }
                
            // *0000 00*00 0000* 0000 00*00 now on the third base64 byte
            let byte = (hex[i+2] << 2) + (hex[i+3] >> 2);
            ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(byte)]);

            if i+4 >= hex.len() {
                panic!("Must be Hexadecimal String! (two characters per hexadecimal value)")
            }

            // *0000 00*00 0000* 0000 00*00 0000* now on the fourth base64 byte
            let prev = hex[i+3] << 6;
            let byte = (prev >> 2) + (hex[i+4]);
            ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(byte)]);

            i=i+6;
        }
        ans
    }

    //when you convert a string to bytes, it returns as base 10, needs to be converted to base 16 for hexadecimal strings
    //all inputs into this function assume they have valid hex characters, will PANIC otherwise
    fn string_to_hex(mut bytes: Vec<u8>) -> Vec<u8> {
        for i in 0..bytes.len() {
            if bytes[i] >=97  && bytes[i] <= 102 {
                match bytes[i] {
                    97 => bytes[i] = 10,
                    98 => bytes[i] = 11,
                    99 => bytes[i] = 12,
                    100 => bytes[i] = 13,
                    101 => bytes[i] = 14,
                    102 => bytes[i] = 15,
                    _ => ()
                }
            }
            else if bytes[i] >= 48  && bytes[i] <= 57 {
                match bytes[i] {
                    48 => bytes[i] = 0,
                    49 => bytes[i] = 1,
                    50 => bytes[i] = 2,
                    51 => bytes[i] = 3,
                    52 => bytes[i] = 4,
                    53 => bytes[i] = 5,
                    54 => bytes[i] = 6,
                    55 => bytes[i] = 7,
                    56 => bytes[i] = 8,
                    57 => bytes[i] = 9,
                    _ => ()
                }
            }
            else {
                panic!("String contains non-hex characters!")
            }
        }
        bytes
    }

    //converts hexadecimal u8 to ascii values for readability.
    fn hex_for_strings(mut bytes: Vec<u8>) -> Vec<u8> {
        for i in 0..bytes.len() {
            if bytes[i] >=10  && bytes[i] <= 15 {
                match bytes[i] {
                    10 => bytes[i] = 97,
                    11 => bytes[i] = 98,
                    12 => bytes[i] = 99,
                    13 => bytes[i] = 100,
                    14 => bytes[i] = 101,
                    15 => bytes[i] = 102,
                    _ => ()
                }
            }
            else if bytes[i] <= 9 {
                match bytes[i] {
                    0 => bytes[i] = 48,
                    1 => bytes[i] = 49,
                    2 => bytes[i] = 50,
                    3 => bytes[i] = 51,
                    4 => bytes[i] = 52,
                    5 => bytes[i] = 53,
                    6 => bytes[i] = 54,
                    7 => bytes[i] = 55,
                    8 => bytes[i] = 56,
                    9 => bytes[i] = 57,
                    _ => ()
                }
            }
            else {
                panic!("String contains non-hex characters!")
            }
        }
        bytes
    }

    //base 16 to base 10 is made by multiplying the first byte by 16 and adding the value of the second byte.
    //if there is no second byte then the entire last base 10 byte will be ignored! Good Riddance!
    fn hex_to_dec(bytes: Vec<u8>) -> Vec<u8> {
        let mut i = 1;
        let mut dec: Vec<u8> = Vec::new();
        while i < bytes.len() {
            //take behind 1 and current to form a complete byte.
            //we are going to use this to form a decimal value
            let temp = (bytes[i-1] * 16u8) + (bytes[i]);
            dec.push(temp);
            i=i+2
        }
        dec
    }

    //used to decrypt a single character xor to a hex string
    //takes a hex string, converts it to u8 hex, converts that to u8 decimal, XOR's the result by ascii 
    //characters from 32 to 126 and finds the most likely decrypted string based on character frequency
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
                for f in 0..Convert::VOWELS.len() {
                    if temp[j] == Convert::VOWELS[f] as u8 {
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
        let converted = Convert::hex_for_strings(xored);
        ans.push_str(std::str::from_utf8(&converted).unwrap());
        ans
    }

    //takes a file location of a list of new line seperated single byte XOR ciphered hex strings, finds the message.
    pub fn detect_xor(string: &str) -> String{
        let mut variations: Vec<String> = Vec::new();

        let content = File::open(string).expect("Unable to open file");
        let content = BufReader::new(content);
        //adds to a vector all of the highest likely deciphered strings for each hex string
        for line in content.lines() {
            let dec = Convert::singlebyte_xor_cipher(&line.as_ref().unwrap());
            variations.push(dec);
        }
        let mut highest = 0;
        let mut position = 0;
        //finds the highest likely deciphered string amongst all the deciphered strings.
        for i in 0..variations.len() {
            let mut count = 0;
            for j in 0..variations[i].len() {
                let temp = variations[i].chars().nth(j).unwrap() as u8;
                //here we get a tally to determine which is the decrypted string based on vowel count
                for f in 0..Convert::VOWELS.len() {
                    if temp == Convert::VOWELS[f] as u8 || temp == 32{
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

}


fn main() {
    //challenge 1
    println!("Answer for Challenge 1: {}", Convert::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
    //challenge 2
    println!("Answer for Challenge 2: {}", Convert::fixed_xor("1c0111001f010100061a024b53535009181c","686974207468652062756c6c277320657965"));
    //challenge 3
    println!("Answer for Challenge 3: {}", Convert::singlebyte_xor_cipher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
    //challenge 4
    println!("Answer for Challenge 4: {}", Convert::detect_xor("./src/challengefour.txt"));
}