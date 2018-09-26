extern crate cryptopals;

use std::iter;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use cryptopals::byte_array::xor;
use cryptopals::frequency;
use cryptopals::hex;

fn main() {
    let file = File::open("data/challenge_1-4.txt").unwrap();
    let reader = BufReader::new(&file);

    let results = reader
        .lines()
        .map(|line| crack_xor(&hex::to_bytes(&line.unwrap())))
        .flatten()
        .collect::<Vec<String>>();

    println!("{:?}", results);
}

fn single_byte_xor_decrypt(chr: u8, cipher: &[u8]) -> Option<String> {
    let key = iter::repeat(chr).take(cipher.len()).collect::<Vec<u8>>();
    let decrypted = xor(&cipher, &key);

    match String::from_utf8(decrypted) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

fn crack_xor(cipher: &[u8]) -> Vec<String> {
    let alphabet = 0..255u8; /* ascii/utf-8 range */

    alphabet
        .into_iter()
        .filter_map(|c| single_byte_xor_decrypt(c, &cipher))
        .filter(|s| frequency::english(s))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analysis_matches_bacon_message() {
        let secret =
            hex::to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

        let actual = crack_xor(&secret);

        assert_eq!(vec!["Cooking MC's like a pound of bacon"], actual);
    }
}
