[![Crates.io][crates-io-badge]][crates-io-url]
[![Docs.rs][docs-badge]][docs-url]
[![MIT License][license-badge]][license-url]

[crates-io-url]: https://crates.io/crates/palindromeda

[crates-io-badge]: https://img.shields.io/crates/v/palindromeda.svg?style=for-the-badge&color=purple

[docs-url]: https://docs.rs/palindromeda/latest/palindromeda/

[docs-badge]: https://img.shields.io/badge/Docs-rs-purple.svg?style=for-the-badge&logo=docsdotrs

[license-url]: https://github.com/CaelusV/palindromeda/blob/master/LICENSE

[license-badge]: https://img.shields.io/badge/License-MIT-purple.svg?style=for-the-badge

# Palindrome generator and checker for numbers.
A palindrome is a letter, number or any other sequence that is the exact same forwards and backwards.
This crate is specifically for palindromic numbers.

## Checking for palindromes
If you want to check whether an **unsigned integer** is a palindrome,
use the `is_palindrome` function:
```rust
let pal1: u64 = 8008; // This is a palindrome.
println!("Is {pal1} a palindrome? {}", pal1.is_palindrome());

let pal2: u8 = 69; // This is NOT a palindrome.
println!("Is {pal2} a palindrome? {}", pal2.is_palindrome());
```
Output:
```text
Is 8008 a palindrome? true
Is 69 a palindrome? false
```

## Generating palindromes
Generating a palindrome is as easy as using either `Palindrome::le`,
`Palindrome::ge` or `Palindrome::closest` for the nearest palindrome
to a number, or by retrieving it based on its palindrome-index
with `Palindrome::nth`:
```rust
use palindromeda::Palindrome;

let number1: u64 = 420; // This number is too high.
// Let's get a palindrome that's lower.
println!("Palindrome that's lower: {}", Palindrome::le(number1));

let number2: u64 = 1337;
// Let's get a palindrome that's higher.
println!("Palindrome that's higher: {}", Palindrome::ge(number2));

let number3: u64 = 5340; // Which palindrome is closest?
println!("Closest palindrome: {}", Palindrome::closest(number3));

let number4: usize = 1000; // 1001st palindrome (0-based indexing)
println!("1001st palindrome: {}", Palindrome::nth(number4).unwrap());
```
Output:
```text
Palindrome that's lower: 414
Palindrome that's higher: 1441
Closest palindrome: 5335
1001st palindrome: 90109
```
And if you want, you can go from palindrome to palindrome with the
`Palindrome::previous` and `Palindrome::next` functions.

## Iterating over palindromes
With `PalindromeIter` you can iterate over a large swathe of palindromes.
You can iterate over a custom range with `PalindromeIter::from_u64` or
iterate over the first `n` palindromes with `PalindromeIter::first_n`.

You can also iterate over the first `n` palindromes after (and including)
a specific palindrome with `PalindromeIter::first_n_from`.
Be sure to use `PalindromeIter::len` for quickly determining the
length of the iterator.
