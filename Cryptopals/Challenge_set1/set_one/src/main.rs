

pub struct Convert;


//cryptopals rule: Always operate on raw bytes, not on enconded strings

impl Convert {
    const BASE64_LOOKUP_TABLE: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                                             'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                                             'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                                             'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/' ];

    const PADDING: char = '=';
    pub fn hex_to_base64(string: &str) -> String {
        let bytes = string.to_lowercase().as_bytes().to_vec();
        let hex = Convert::dec_to_hex(bytes);
        let mut ans = String::new();
        let mut i = 1;
        while i < hex.len() {
            let byte = (hex[i-1] << 2) + (hex[i] >> 2);
            ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(byte)]);

            if i+1 >= hex.len() {
                ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(hex[i] << 4)]);
                ans.push(Convert::PADDING);
                ans.push(Convert::PADDING);
                break;
            }

            let prev = hex[i] << 6;
            let byte = (prev >> 2) + (hex[i+1]);
            ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(byte)]);

            if i+2 >= hex.len() {
                panic!("Must be Hexadecimal String! (two characters per hexadecimal value)")
            }

            if i+3 >= hex.len() {
                ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(hex[i+2] << 2)]);
                ans.push(Convert::PADDING);
                break;
            }
                
            let byte = (hex[i+2] << 2) + (hex[i+3] >> 2);
            ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(byte)]);

            if i+4 >= hex.len() {
                panic!("Must be Hexadecimal String! (two characters per hexadecimal value)")
            }

            let prev = hex[i+3] << 6;
            let byte = (prev >> 2) + (hex[i+4]);
            ans.push(Convert::BASE64_LOOKUP_TABLE[usize::from(byte)]);

            i=i+6;
        }
        ans
        //base64 000000 vs.
        //hex    00000000
        //need to bitshift previous byte right twice, grab that as a base64, go to current byte, and bitshift previous byte left 4 times 
        //and bitshift current byte right twice and combine, then increment to next byte and repeat this cycle.
    }

    //when you convert a string to bytes, it returns as base 10, needs to be converted to base 16
    //all inputs into this function assume they have valid hex characters
    fn dec_to_hex(mut bytes: Vec<u8>) -> Vec<u8> {
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

    fn hex_to_dec(mut bytes: Vec<u8>) -> Vec<u8> {
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
            else if bytes[i] >= 0  && bytes[i] <= 9 {
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

    pub fn fixed_xor(string1: &str, string2: &str) -> String {
        if string1.len() != string2.len() {
            panic!("The strings must be equal length!");
        }
        let hex1 = Convert::dec_to_hex(string1.to_lowercase().as_bytes().to_vec());
        let hex2 = Convert::dec_to_hex(string2.to_lowercase().as_bytes().to_vec());
        let mut xored: Vec<u8> = Vec::new();
        let mut ans = String::new();
        for i in 0..hex1.len() {
            xored.push(hex1[i] ^ hex2[i]);
        }
        let converted = Convert::hex_to_dec(xored);
        ans.push_str(std::str::from_utf8(&converted).unwrap());
        ans
    }

}


fn main() {
    println!("{}", Convert::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
    println!("{}", Convert::hex_to_base64("920391"));
    println!("{}", Convert::hex_to_base64("1c0111001f010100061a024b53535009181c"));
    println!("{}", Convert::fixed_xor("1c0111001f010100061a024b53535009181c","686974207468652062756c6c277320657965"));
}