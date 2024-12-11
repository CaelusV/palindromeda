# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2024-12-11

### Changed

- Renamed PalindromeIter::first_n_palindromes -> PalindromeIter::first_n
- Renamed PalindromeIter::first_n_palindromes_from -> PalindromeIter::first_n_from
- Palindrome::{MIN, MAX} are now of type Palindrome
- Optimized Palindrome::len

### Added

- Added Palindrome::nth
- Added Palindrome::to_n
- Conversion from &Palindrome for u64

### Fixed

- Fixed comparison of Palindrome causing stack overflow

## [0.3.0] - 2024-12-10

### Added

- Added PalindromeIter::first_n_palindromes to get the first `n` palindromes from 0.
- Added PalindromeIter::first_n_palindromes_from to get the first `n` palindromes from a palindrome.

## [0.2.1] - 2024-12-09

### Fixed

- Removed leftover println statement

## [0.2.0] - 2024-12-09

### Added

- Added PalindromeIter::len to efficiently calculate the length of the iterator.
- Added Palindrome::closest to get the closest palindrome to an argument.

### Fixed

- Fix Palindrome::le and Palindrome::ge generating incorrect Palindromes in some instances.

## [0.1.0] - 2024-12-07
First release
