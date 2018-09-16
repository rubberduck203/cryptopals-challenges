# Cryptopals Challenge

http://cryptopals.com/


## Challenges Completed

- [x] [Convert hex to base64](http://cryptopals.com/sets/1/challenges/1)
- [x] [Fixed XOR](http://cryptopals.com/sets/1/challenges/2)
- [ ] [Single-byte XOR cipher](http://cryptopals.com/sets/1/challenges/3)

## TODO

- [ ] Extract `hex` module into it's own file

## Notes

### Set 1/Challenge 3

See [Xor Cipher on Wikipedia](https://en.wikipedia.org/wiki/XOR_cipher).

First, I brute forced the decryption by trying every key in the extended ascii range.

```
for char in alphabet
    let key_to_try = repeat(char, cipher.len)
    xor(key_to_try, cipher)
```

Then I visually grepped the results for anything that didnt look like gibberish.
I spotted this line.

```
121  Ok("cOOKING\u{0}mc\u{7}S\u{0}LIKE\u{0}A\u{0}POUND\u{0}OF\u{0}BACON")
```

This is close enough to read the message, but not quite right,
which prompted me to do some actual grepping.

```
> cargo run | nl | grep bacon
    89  Ok("Cooking MC\'s like a pound of bacon")
```

Bingo! 
Now I have a test case for implementing [frequency analysis](https://en.wikipedia.org/wiki/Frequency_analysis).
