pub struct Convert;

impl Convert {

    //base64 table with each value indexed to it's binary value
    const BASE64_LOOKUP_TABLE: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                                             'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                                             'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                                             'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/' ];
    //base64 has padding characters when there aren't 3 bytes (24 bits) to work with. 
    const PADDING: char = '=';


    //converts any decimal to hex
    pub fn dec_to_hex(dec: Vec<u8>) -> Vec<u8> {
        let mut hex: Vec<u8> = Vec::new();
        for i in 0..dec.len() {
            let mut quotient = dec[i].clone();
            let mut conversion = 0;
            conversion = (quotient/16) % 16;
            hex.push(conversion);
            conversion = quotient % 16;
            hex.push(conversion);
        }
        hex
    }

    //base 16 to base 10 is made by multiplying the first byte by 16 and adding the value of the second byte.
    //if there is no second byte then the entire last base 10 byte will be ignored! Good Riddance!
    pub fn hex_to_dec(bytes: Vec<u8>) -> Vec<u8> {
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

    //converts hexadecimal u8 to ascii values for readability.
    pub fn hex_to_string(mut bytes: Vec<u8>) -> Vec<u8> {
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

    //when you convert a string to bytes, it returns as base 10, needs to be converted to base 16 for hexadecimal strings
    //all inputs into this function assume they have valid hex characters, will PANIC otherwise
    pub fn string_to_hex(mut bytes: Vec<u8>) -> Vec<u8> {
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
}

