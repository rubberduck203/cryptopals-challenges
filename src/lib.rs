pub mod frequency;
pub mod hex;

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
