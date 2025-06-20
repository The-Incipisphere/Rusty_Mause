# Rusty Mause

a small CLI tool that i built to help me learn rust

basically allows you to encode (decoding is not yet implemented) a string into anything you provide in a csv

## how to use:
1) [install rust](https://www.rust-lang.org/tools/install)
2) clone this repository
3) make (or grab) a CSV file with the morse dictionary you wanna use
    - each line should follow the format of `LETTER,MORSE`
4) open up your terminal/commandprompt, navigate to the directory you cloned to, and do `cargo run`
5) paste the path to the file after the `CSV file to read:` prompt
    - if you're on windows and you're pasting the absolute path (`C:\users\xx\..`), change all backslashes (`\`) to forwardslashes (`/`).
    - it **WILL** panic if it cant find the file at the path you specify (in which case, just do `cargo run` again)
6) type in the string you want to encode/~~decode~~ after the `String to decode/encode:` prompt
7) specify whether you'd like to encode (enc) or ~~decode (dec)~~ after the `Decode Or Encode? (dec/enc):` prompt
8) profit. (if something in the input string cannot be found within the CSV dict you provided, it will be replaced with a `#`)
