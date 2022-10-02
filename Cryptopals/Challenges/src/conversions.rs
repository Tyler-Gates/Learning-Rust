pub struct Convert;
use phf::phf_map;

impl Convert {

    //base64 hashmap tables to convert between base10 ascii and their respective base64 values
    const BASE64_TO_CHAR_HASHMAP: phf::Map<u8, char> = phf_map! { 0u8 => 'A',  1u8 => 'B',  2u8 =>  'C', 3u8 => 'D',  4u8 =>  'E', 5u8 =>  'F', 6u8 =>  'G', 7u8 =>  'H', 8u8 =>  'I', 9u8 =>  'J', 10u8 => 'K', 11u8 => 'L', 12u8 => 'M', 13u8 => 'N', 14u8 => 'O', 15u8 => 'P',
                                                                  16u8 => 'Q', 17u8 => 'R', 18u8 => 'S', 19u8 => 'T', 20u8 => 'U', 21u8 => 'V', 22u8 => 'W', 23u8 => 'X', 24u8 => 'Y', 25u8 => 'Z', 26u8 => 'a', 27u8 => 'b', 28u8 => 'c', 29u8 => 'd', 30u8 => 'e', 31u8 => 'f',
                                                                  32u8 => 'g', 33u8 => 'h', 34u8 => 'i', 35u8 => 'j', 36u8 => 'k', 37u8 => 'l', 38u8 => 'm', 39u8 => 'n', 40u8 => 'o', 41u8 => 'p', 42u8 => 'q', 43u8 => 'r', 44u8 => 's', 45u8 => 't', 46u8 => 'u', 47u8 => 'v',
                                                                  48u8 => 'w', 49u8 => 'x', 50u8 => 'y', 51u8 => 'z', 52u8 => '0', 53u8 => '1', 54u8 => '2', 55u8 => '3', 56u8 => '4', 57u8 => '5', 58u8 => '6', 59u8 => '7', 60u8 => '8', 61u8 => '9', 62u8 => '+', 63u8 => '/' };

    const BASE64_TO_BINARY_HASHMAP: phf::Map<char, u8> = phf_map! { 'A' => 0u8,  'B' => 1u8,  'C' => 2u8,  'D' => 3u8,  'E' => 4u8,  'F' => 5u8,  'G' => 6u8,  'H' => 7u8,  'I' => 8u8,  'J' => 9u8,  'K' => 10u8, 'L' => 11u8, 'M' => 12u8, 'N' => 13u8, 'O' => 14u8, 'P' => 15u8,
                                                                    'Q' => 16u8, 'R' => 17u8, 'S' => 18u8, 'T' => 19u8, 'U' => 20u8, 'V' => 21u8, 'W' => 22u8, 'X' => 23u8, 'Y' => 24u8, 'Z' => 25u8, 'a' => 26u8, 'b' => 27u8, 'c' => 28u8, 'd' => 29u8, 'e' => 30u8, 'f' => 31u8,
                                                                    'g' => 32u8, 'h' => 33u8, 'i' => 34u8, 'j' => 35u8, 'k' => 36u8, 'l' => 37u8, 'm' => 38u8, 'n' => 39u8, 'o' => 40u8, 'p' => 41u8, 'q' => 42u8, 'r' => 43u8, 's' => 44u8, 't' => 45u8, 'u' => 46u8, 'v' => 47u8,
                                                                    'w' => 48u8, 'x' => 49u8, 'y' => 50u8, 'z' => 51u8, '0' => 52u8, '1' => 53u8, '2' => 54u8, '3' => 55u8, '4' => 56u8, '5' => 57u8, '6' => 58u8, '7' => 59u8, '8' => 60u8, '9' => 61u8, '+' => 62u8, '/' => 63u8 };
    //base64 has padding characters when there aren't 3 bytes (24 bits) to work with. 
    const PADDING: char = '=';


    //converts any decimal to hex
    pub fn dec_to_hex(dec: Vec<u8>) -> Vec<u8> {
        let mut hex: Vec<u8> = Vec::new();
        for i in 0..dec.len() {
            let quotient = dec[i].clone();
            let conversion = (((quotient/16) % 16) << 4) + ((quotient % 16) >> 4);
            hex.push(conversion);
            println!("dec: {}  hex: {}", quotient, conversion);
        }
        hex
    }

    //base 16 to base 10 is made by multiplying the first byte by 16 and adding the value of the second byte.
    //if there is no second byte then the entire last base 10 byte will be ignored! Good Riddance!
    pub fn hex_to_dec(bytes: Vec<u8>) -> Vec<u8> {
        let mut i = 1;
        let mut dec: Vec<u8> = Vec::new();
        for i in 0..bytes.len() {
            //take behind 1 and current to form a complete byte.
            //we are going to use this to form a decimal value
            let temp = ((bytes[i]>> 4) * 16u8) + ((bytes[i] << 4) >> 4);
            dec.push(temp);
        }
        dec
    }

    //converts hexadecimal u8 to ascii values for readability.
    pub fn hex_to_string(mut bytes: Vec<u8>) -> Vec<u8> {
        let mut hex: Vec<u8> = Vec::new();
        let mut i = 0;
        let mut bits = 0u8;
        let mut left = true;
        while i < bytes.len() {
            bits = if left {bytes[i] >> 4} else {(bytes[i] << 4) >> 4};
            if bits >= 10u8  && bits <= 15u8 {
                match bits {
                    10u8 => hex.push(97u8),
                    11u8 => hex.push(98u8),
                    12u8 => hex.push(99u8),
                    13u8 => hex.push(100u8),
                    14u8 => hex.push(101u8),
                    15u8 => hex.push(102u8),
                    _ => ()
                }
            }
            else if bits <= 9u8 {
                match bits {
                    0u8 => hex.push(48u8),
                    1u8 => hex.push(49u8),
                    2u8 => hex.push(50u8),
                    3u8 => hex.push(51u8),
                    4u8 => hex.push(52u8),
                    5u8 => hex.push(53u8),
                    6u8 => hex.push(54u8),
                    7u8 => hex.push(55u8),
                    8u8 => hex.push(56u8),
                    9u8 => hex.push(57u8),
                    _ => ()
                }
            }
            else {
                panic!("String contains non-hex characters!")
            }
            if !left {
                i = i +1;
            }
            left = !left;
        }
        hex
    }

    pub fn string_to_hex_helper(mut temp: u8, iterator: &usize, value: u8) -> u8 {
        if iterator % 2 == 0 {
            temp = temp + (value << 4);
        }
        else {
            temp = temp + value;
        }
        temp
    }

    //convert hex encoded string to actual hex
    pub fn string_to_hex(mut bytes: Vec<u8>) -> Vec<u8> {
        let mut hex: Vec<u8> = Vec::new();
        let mut temp = 0u8;
        for i in 0..bytes.len() {
            if bytes[i] >=97  && bytes[i] <= 102 {
                match bytes[i] {
                    97 => temp = Convert::string_to_hex_helper(temp,&i,10),
                    98 => temp = Convert::string_to_hex_helper(temp,&i,11),
                    99 => temp = Convert::string_to_hex_helper(temp,&i,12),
                    100 => temp = Convert::string_to_hex_helper(temp,&i,13),
                    101 => temp = Convert::string_to_hex_helper(temp,&i,14),
                    102 => temp = Convert::string_to_hex_helper(temp,&i,15),
                    _ => ()
                }
            }
            else if bytes[i] >= 48  && bytes[i] <= 57 {
                match bytes[i] {
                    48 => temp = Convert::string_to_hex_helper(temp,&i,0),
                    49 => temp = Convert::string_to_hex_helper(temp,&i,1),
                    50 => temp = Convert::string_to_hex_helper(temp,&i,2),
                    51 => temp = Convert::string_to_hex_helper(temp,&i,3),
                    52 => temp = Convert::string_to_hex_helper(temp,&i,4),
                    53 => temp = Convert::string_to_hex_helper(temp,&i,5),
                    54 => temp = Convert::string_to_hex_helper(temp,&i,6),
                    55 => temp = Convert::string_to_hex_helper(temp,&i,7),
                    56 => temp = Convert::string_to_hex_helper(temp,&i,8),
                    57 => temp = Convert::string_to_hex_helper(temp,&i,9),
                    _ => ()
                }
            }
            else {
                panic!("String contains non-hex characters!");
            }
            if i % 2 != 0 {
                hex.push(temp);
                temp = 0u8;
            }
            else if i == bytes.len()-1{
                panic!("Must be hexadecimal String!");
            }
        }
        hex
    }

    //Takes a string that is Hex encoded and converts it to a correct hex byte medium then is bit manipulated into base64.
    //Will panic is parameter string isn't a valid hexadecimal.
    pub fn hex_to_base64(string: &str) -> String {
        let bytes = string.to_lowercase().as_bytes().to_vec();
        let hex = Convert::string_to_hex(bytes);

        let mut ans = String::new();
        let mut i = 0;
        while i < hex.len() {

            // *000000*00 
            let byte = (hex[i] >> 2);
            ans.push(Convert::BASE64_TO_CHAR_HASHMAP[&(byte)]);

            //if the end is reached, utilize the remaining bits, add padding to 24 bits, and break.
            if i+1 >= hex.len() {
                ans.push(Convert::BASE64_TO_CHAR_HASHMAP[&((hex[i] << 6) >> 2)]);
                ans.push(Convert::PADDING);
                ans.push(Convert::PADDING);
                break;
            }

            // *000000*00 0000*0000
            let prev = (hex[i] << 6) >> 2;
            let byte = prev + (hex[i+1] >> 4);
            ans.push(Convert::BASE64_TO_CHAR_HASHMAP[&byte]);

            //if the end is reached, utilize the remaining bits, add padding to 24 bits, and break.
            if i+2 >= hex.len() {
                ans.push(Convert::BASE64_TO_CHAR_HASHMAP[&((hex[i+1] << 4) >> 2)]);
                ans.push(Convert::PADDING);
                break;
            }

            // *000000*00 0000*0000 00*000000
            let byte = ((hex[i+1] << 4) >> 2) + (hex[i+2] >> 6);
            ans.push(Convert::BASE64_TO_CHAR_HASHMAP[&byte]);

            // *000000*00 0000*0000 00*000000
            let byte = (hex[i+2] << 2) >> 2;
            ans.push(Convert::BASE64_TO_CHAR_HASHMAP[&byte]);

            i=i+3;
        }
        ans
    }

    //converts base64 string to ascii string
    pub fn base64_to_chars(string: &str) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        let mut ans = String::new();
        for i in 0..string.len() {
            let current = string.chars().nth(i).unwrap();
            if Convert::BASE64_TO_BINARY_HASHMAP.contains_key(&current) {
                bytes.push(Convert::BASE64_TO_BINARY_HASHMAP[&current]);
            }
            else if current != '=' {
                panic!("Not a valid base64 string!");
            }
        }
        let mut index = 1;
        while index < bytes.len() {
            let byte = ((bytes[index-1] << 2) + (bytes[index] >> 4)) as u8;
            ans.push(byte as char);

            if index+1 >= bytes.len() {
                if  (bytes[index] << 4) != 0u8 {
                    panic!("Not a valid base64 string, last four bits should be 0..");
                }
                break;
            }

            let byte = ((bytes[index] << 4) + (bytes[index+1] >> 2)) as u8;
            ans.push(byte as char);

            if index+2 >= bytes.len() {
                if  (bytes[index+1] << 6) != 0u8 {
                    panic!("Not a valid base64 string, last two bits should be 0..");
                }
                break;
            }
            let byte = ((bytes[index+1] << 6) + (bytes[index+2])) as u8;
            ans.push(byte as char);
            index = index + 4;
        }
        ans
    }
}

