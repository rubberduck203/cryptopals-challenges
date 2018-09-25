extern crate base64;
use std::str;

pub fn to_base64(hex: &str) -> String {
    base64::encode(&to_bytes(hex))
}

pub fn to_bytes(hex: &str) -> Vec<u8> {
    assert!(
        hex.len() % 2 == 0,
        "Hex strings must have an even number of characters."
    );

    hex.as_bytes()
        .chunks(2) // each char represents one nibble
        .map(|nibbles| str::from_utf8(&nibbles).unwrap())
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_hex_to_base64() {
        let hex_as_string =
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64_as_string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(to_base64(hex_as_string), base64_as_string);
    }

    #[test]
    fn ff_byte_hex_string_to_byte_vector() {
        let hex = "FF";

        assert_eq!(vec![0xFF], to_bytes(hex));
    }

    #[test]
    fn single_byte_hex_string_to_byte_vector() {
        let hex = "2A";
        assert_eq!(vec![0x2A], to_bytes(hex));
    }

    #[test]
    fn multibyte_hex_string_to_byte_vector() {
        let hex = "2ABE";
        assert_eq!(vec![0x2A, 0xBE], to_bytes(hex));
    }

    #[test]
    #[should_panic]
    fn odd_length_string_is_not_hex() {
        to_bytes("2AC");
    }
}
