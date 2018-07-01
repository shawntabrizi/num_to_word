# num_to_word in Rust
Convert a number to to its written english form using Rust.

## Problem Statement
> Could you write a Rust program that accepts a numeric literal through stdin and writes out a human-readable english form of it?
>
> e.g. 1355823 -> "one million, three hundred fifty five thousand, eight hundred twenty three” 

## Build Instructions
* Ensure you have [Rust](https://www.rust-lang.org/en-US/install.html) and [Cargo](https://crates.io/) installed
* Download this repository
* In command line, type `cargo run` in the root folder

## Testing and Validation
```
C:\Users\shawn\Documents\RUST\num_to_word>cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\num_to_word.exe`
     
Please input a 32-bit integer:
1355823
one million, three hundred fifty five thousand, eight hundred twenty three

Please input a 32-bit integer:
0
zero

Please input a 32-bit integer:
1
one

Please input a 32-bit integer:
11
eleven

Please input a 32-bit integer:
100000
one hundred thousand

Please input a 32-bit integer:
1234567890
one billion, two hundred thirty four million, five hundred sixty seven thousand, eight hundred ninety

Please input a 32-bit integer:
1a234asda
Couldn't parse that value... Try again.

Please input a 32-bit integer:
1,234,567
one million, two hundred thirty four thousand, five hundred sixty seven

Please input a 32-bit integer:
2,147,483,647
two billion, one hundred fourty seven million, four hundred eighty three thousand, six hundred fourty seven

Please input a 32-bit integer:
-2,147,483,648
negative two billion, one hundred fourty seven million, four hundred eighty three thousand, six hundred fourty eight

Please input a 32-bit integer:
2,147,483,648
Couldn't parse that value... Try again.
```

