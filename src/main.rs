extern crate cryptopals;
use cryptopals::byte_array::xor;
use cryptopals::hex;
use std::iter;
use std::str;

fn main() {
    let secret =
        hex::to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let alphabet = 0..255u8; /* ascii/utf-8 range */

    for c in alphabet {
        let key = iter::repeat(c).take(secret.len()).collect::<Vec<u8>>();
        let decrypted = xor(&secret, &key);

        println!("{:?}", str::from_utf8(&decrypted));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
