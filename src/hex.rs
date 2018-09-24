extern crate base64;

pub fn to_base64(hex: &str) -> String {
    base64::encode(&to_bytes(hex))
}

pub fn to_bytes(hex: &str) -> Vec<u8> {
    hex.chars()
            .collect::<Vec<_>>() //convert to slice-able
            .chunks(2)           //each char is one nibble
            .map(|byte| byte.iter().collect::<String>())
            .map(|byte| u8::from_str_radix(&byte[..], 16).unwrap())
            .collect()
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
}
