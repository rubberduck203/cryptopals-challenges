extern crate cryptopals;
use cryptopals::byte_array::xor;
use cryptopals::frequency;
use cryptopals::hex;

use std::iter;

fn main() {
    let secret =
        hex::to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    println!("{:?}", crack(&secret));
}

fn crack(cipher: &[u8]) -> Vec<String> {
    let alphabet = 0..255u8; /* ascii/utf-8 range */

    alphabet
        .into_iter()
        .filter_map(|c| {
            let key = iter::repeat(c).take(cipher.len()).collect::<Vec<u8>>();
            let decrypted = xor(&cipher, &key);

            match String::from_utf8(decrypted) {
                Ok(s) => Some(s),
                Err(_) => None,
            }
        }).filter(|s| frequency::english(s))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analysis_matches_bacon_message() {
        let secret =
            hex::to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

        let actual = crack(&secret);

        assert_eq!(vec!["Cooking MC's like a pound of bacon"], actual);
    }
}
