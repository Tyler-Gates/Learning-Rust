use crate::conversions::Convert;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Challenges;


//cryptopals rule: Always operate on raw bytes, not on enconded strings

impl Challenges {

    //used to decrypt a hex string that has been XOR'd against a single character to reveal a hidden message..
    //this takes a hex string, converts it to u8 hex vector, converts that to a u8 decimal vector, XOR's the result
    //from characters 32 to 126 and finds the most likely decrypted string based on character frequency
    pub fn singlebyte_xor_cipher(string: &Vec<u8>) -> (String, u8){
        let mut variations: Vec<String> = Vec::new();
        let mut highest = 0;
        let mut position = 0;
        for i in 32..127 {
            let mut temp: Vec<u8> = Vec::new();
            let mut count = 0;
            for j in  0..string.len() {
                //XORs the decimal value and wraps it around 32..127 until it's withing range of a printable character
                let mut xor = string[j] ^ i;
                if xor < 32 {
                    xor = xor + 32;
                }
                while xor > 127 {
                    xor = xor - 95;
                }
                temp.push(xor);
            }
            //here we get a tally to determine which is the decrypted string based on letter / space count
            for j in 0..temp.len() {
                if (temp[j] >= 65 && temp[j] <=90) || (temp[j] >= 97 && temp[j] <= 122) || temp[j] == 63 || (temp[j] >= 44 && temp[j] <= 46) || (temp[j] >= 32 && temp[j] <= 34){
                   count = count + 1;
                }
                //and take away a tally if it has characters not commonly used in sentences.
                if (temp[j] >= 91 && temp[j] <= 96) || temp[j] == 47 || temp[j] == 64 ||(temp[j] >= 58 && temp[j] <= 62) || (temp[j] >= 35 && temp[j] <=  43) || (temp[j] >= 123 && temp[j] <=  127) {
                    count = count - 2;
                }
            }
            if highest < count {
                highest = count;
                position = i-32;
            }
            variations.push(std::string::String::from_utf8(temp).unwrap());
        }
        (variations.remove(position.into()), position+32)

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
            let bytes = line.unwrap().to_lowercase().as_bytes().to_vec();
            let dec = Convert::hex_to_dec(Convert::string_to_hex(bytes));
            let (dec,_) = Challenges::singlebyte_xor_cipher(&dec);
            variations.push(dec);
        }
        //finds the highest likely deciphered string amongst all the deciphered strings.
        for i in 0..variations.len() {
            let mut count = 0;
            for j in 0..variations[i].len() {
                let temp = variations[i].chars().nth(j).unwrap() as u8;
                //here we get a tally to determine which is the decrypted string based on letter / space count
                if (temp >= 65 && temp <=90) || (temp >= 97 && temp <= 122) || temp == 32{
                    count = count + 3;
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

    pub fn edit_distance_calculation(string1: &str, string2: &str) -> u32 {
        let mut count = 0;
        if string1.len() != string2.len() {
            panic!("Inequal string lengths for hamming distance calculation!");
        }
        for i in 0..string1.len() {
            let char1 = string1.chars().nth(i).unwrap() as u8;
            let char2 = string2.chars().nth(i).unwrap() as u8;
            if char1 >= 32 && char1 <= 127 && char2 >= 32 && char2 <= 127 {
                count = count + (char1 ^ char2).count_ones();
            }
            else {
                count = count + ((char1 +32u8) ^ (char2+32u8)).count_ones();
            }
        }
        count
    }

    pub fn break_repeatkey_xor(string: &str) -> String{
        let converted = Convert::base64_to_chars(string);
        let mut lowest_distance = f32::MAX;
        let mut key = String::new();
        let mut keysize = 0;
        for i in 2..=40 {
            let slice1 = &converted[0..i];
            let slice2 = &converted[i..(i*2)];
            let slice3 = &converted[(i*2)..(i*3)];
            let slice4 = &converted[(i*4)..(i*5)];
            let distance = ((Challenges::edit_distance_calculation(slice1,slice2) as f32) + (Challenges::edit_distance_calculation(slice1,slice3) as f32) + (Challenges::edit_distance_calculation(slice1,slice4) as f32) 
                          + (Challenges::edit_distance_calculation(slice2,slice3) as f32) + (Challenges::edit_distance_calculation(slice2,slice4) as f32) + (Challenges::edit_distance_calculation(slice3,slice4) as f32)) / (6f32 * i as f32);
            
            println!("Position: {}   distance: {}  slice1: {:#?}    slice2: {:#?}", i, distance, slice1, slice2);
            if lowest_distance >= distance {
                lowest_distance = distance;
                keysize = i;
            }
        }
        println!("keysize: {}\n converted length: {}", keysize, converted.len());
        //keysize = 29;
        let mut xored: Vec<u8> = Vec::new();
        let mut blocks: Vec<String> = Vec::new();
        for i in 0..keysize {
            for j in 0..converted.len()/keysize {
                xored.push(converted.chars().nth((j*keysize)+i).unwrap() as u8);
                if (j == converted.len()/keysize - 1) && (((j+1)*keysize) + i < converted.len()) {
                    xored.push(converted.chars().nth(((j+1)*keysize) + i).unwrap() as u8);
                }
            }
            let (x,y) = Challenges::singlebyte_xor_cipher(&xored);
            println!("block {}: {}", i, x);
            blocks.push(x);
            key.push(y as char);
            xored.clear();
        }
        let mut ans = String::new();
        for i in 0..blocks[0].len() {
            for j in 0..keysize {
                if blocks[j].len() > i {
                    ans.push(blocks[j].chars().nth(i).unwrap());
                }
            }
        }
        println!("key: {:#?}", key);

        ans
    }
}