# Cryptopals Challenge

http://cryptopals.com/


## Challenges Completed

- [x] [Convert hex to base64](http://cryptopals.com/sets/1/challenges/1)
- [x] [Fixed XOR](http://cryptopals.com/sets/1/challenges/2)
- [x] [Single-byte XOR cipher](http://cryptopals.com/sets/1/challenges/3)
- [ ] [Detect single-character XOR](http://cryptopals.com/sets/1/challenges/4)

## TODO

- [x] Extract `hex` module into it's own file
- [x] Extract frequency analysis data into file and code gen at build time

## Frequency Analysis Data

The dataset I use to perform frequency analysis, [data/english.csv](data/english.csv) was taken from  http://www.fitaly.com/board/domper3/posts/136.html

See [build.rs](build.rs) for how this dataset is used via code generation.

