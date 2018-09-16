pub mod byte_array {
    pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use hex;

        #[test]
        fn xor_two_equal_length_byte_arrays() {
            let hex_a = hex::to_bytes("1c0111001f010100061a024b53535009181c");
            let hex_b = hex::to_bytes("686974207468652062756c6c277320657965");

            assert_eq!(
                hex::to_bytes("746865206b696420646f6e277420706c6179"),
                xor(&hex_a, &hex_b),
            );
        }
    }
}

pub mod hex {
    extern crate base64;
    use self::base64::encode;

    pub fn to_base64(hex: &str) -> String {
        encode(&to_bytes(hex))
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
            let base64_as_string =
                "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

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
}
