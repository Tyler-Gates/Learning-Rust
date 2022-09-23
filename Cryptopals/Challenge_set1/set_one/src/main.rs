

pub struct Convert;

impl Convert {

    //cryptopals rule: Always operate on raw bytes, not on enconded strings
    pub fn hex_to_base64(hex: &str){
        let bytes = hex.as_bytes();
        let mut i = 0;
        while i < bytes.len(){
            let grouped = bytes[i] + bytes[i+1] + bytes[i+2] + bytes[i+3] + bytes[i+4] + bytes[i+5];
            println!("grouped: {}", grouped);
        }

    }
}


fn main() {
    //Convert::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("{:#?}", b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
}


