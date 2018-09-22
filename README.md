# Cryptopals Challenge

http://cryptopals.com/


## Challenges Completed

- [x] [Convert hex to base64](http://cryptopals.com/sets/1/challenges/1)
- [x] [Fixed XOR](http://cryptopals.com/sets/1/challenges/2)
- [x] [Single-byte XOR cipher](http://cryptopals.com/sets/1/challenges/3)
- [ ] [Detect single-character XOR](http://cryptopals.com/sets/1/challenges/4)

## TODO

- [x] Extract `hex` module into it's own file
- [ ] Extract frequency analysis data into file and code gen at build time

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

#### Frequency Analysis

In order to automatically detect the correct key, choose a key, decrypt the message with that key,
then run a [chi-squared test][chi-squared] on the [frequency analysis][freq-analysis] on the decypted message.

For the frequency analysis, I need to find a dataset that includes the space character in the analysis. 

[chi-squared]: https://en.wikipedia.org/wiki/Chi-squared_test
[freq-analysis]: https://en.wikipedia.org/wiki/Frequency_analysis

**Chi-squared:**

χ^2 =  Σ( (observed - expected) ^ 2 / expected)

⍺ = 0.05 = 5% (significance factor, can be adjusted as needed)

if χ^2 > ⍺, then it is likely we have an English sentence, and therefore have found the key.

Chi squared cannot work on percentages, so I first need to turn the expected % into an expected count based on the message length.

**Psuedo code:**

expected_counts = 
	expected_percentages.map(|e| -> e * message.length())

acutal_counts = decrypted_message.group_by(char, sum)

distributions = expected_counts.zip(actual_counts.sort_by_value())

chi-statistic = distributions.map(|expected, observed| -> (observed - expected) ^ 2 / expected).sum

P = chi-distribution-table.lookup(chi-statistic)

return P > ⍺

