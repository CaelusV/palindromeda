//! # Palindrome generator and checker for numbers.
//! A palindrome is a letter, number or any other sequence that is the exact same forwards and backwards.
//! This crate is specifically for palindromic numbers.
//!
//! ## Checking for palindromes
//! If you want to check whether an **unsigned integer** is a palindrome,
//! use the [`is_palindrome`](`IsPalindrome::is_palindrome`) function:
//! ```
//! use palindromeda::IsPalindrome;
//!
//! let pal1: u64 = 8008; // This is a palindrome.
//! println!("Is {pal1} a palindrome? {}", pal1.is_palindrome());
//!
//! let pal2: u8 = 69; // This is NOT a palindrome.
//! println!("Is {pal2} a palindrome? {}", pal2.is_palindrome());
//! ```
//! Output:
//! ```text
//! Is 8008 a palindrome? true
//! Is 69 a palindrome? false
//! ```
//!
//! ## Generating palindromes
//! Generating a palindrome is as easy as using either [`Palindrome::le`] or [`Palindrome::ge`]
//! for the nearest palindrome to your number:
//! ```
//! use palindromeda::Palindrome;
//!
//! let number1: u64 = 420; // This number is too high.
//! // Let's get a palindrome that's lower.
//! println!("Palindrome that's lower: {}", Palindrome::le(number1));
//!
//! let number2: u64 = 1337;
//! // Let's get a palindrome that's higher.
//! println!("Palindrome that's higher: {}", Palindrome::ge(number2));
//! ```
//! Output:
//! ```text
//! Palindrome that's lower: 414
//! Palindrome that's higher: 1441
//! ```
//! And if you want, you can go from palindrome to palindrome with the
//! [`Palindrome::previous`] and [`Palindrome::next`] functions.
//!
//! ## Iterating over palindromes
//! With [`PalindromeIter`] you can also iterate over a large swathe of palindromes.

use forward_ref::{forward_ref_binop, forward_ref_op_assign};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
    u64,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    pub const MIN: u64 = 0;
    /// The largest possible palindrome that can fit in a [`std::u64`].
    pub const MAX: u64 = 18_446_744_066_044_764_481;

    /// Return the palindrome closest to `x`.
    ///
    /// **NOTE:** If the closest palindrome is in both directions,
    /// return the higher number. E.g.: `x=10` returns `11`.
    pub fn closest(x: u64) -> Self {
        let ge = Self::ge(x);
        let le = Self::le(x);
        if ge - x <= x - le {
            return ge;
        }

        le
    }

    /// Construct a palindrome from the first half of a digit and a provided length.
    ///
    /// NOTE: Will panic if `length` isn't `2x` or `2x - 1` the size of `digits_half.len()`.
    fn construct_palindrome(length: usize, digits_half: &[u8]) -> Self {
        assert_eq!(
            length.div_ceil(2),
            digits_half.len(),
            "length ({length}) isn't compatible with the size of digits_half ({}). Valid length values: '{}' & '{}'.",
            digits_half.len(), digits_half.len() * 2 - 1, digits_half.len() * 2
        );

        // If we have a 5-digit number, then we construct by using
        // the 1st, 2nd, 3rd, 2nd, and 1st elements.
        // If we have a 6-digit number, then we construct by using
        // the 1st, 2nd, 3rd, 3rd, 2nd, and 1st elements.
        let second_half_range = length - digits_half.len();
        let mut palindrome = 0;
        for fh_idx in 0..digits_half.len() {
            palindrome *= 10;
            palindrome += digits_half[fh_idx] as u64;
        }
        for sh_rev_idx in 1..=second_half_range {
            palindrome *= 10;
            palindrome += digits_half[second_half_range - sh_rev_idx] as u64;
        }

        Palindrome(palindrome)
    }

    /// Return the digits and the length of a number.
    fn digits_and_length(mut x: u64) -> (Vec<u8>, usize) {
        let mut length = 0;
        let mut digits = Vec::new();

        // We use a "do while" loop, to account for when x=0.
        loop {
            digits.push((x % 10) as u8);
            x /= 10;
            length += 1;
            if x == 0 {
                break;
            }
        }
        digits.reverse(); // Reversing isn't that terrible in release mode.
        (digits, length)
    }

    /// Return the previous palindromic number.
    ///
    /// **NOTE:** Lowest return-value is [`Self::MIN`].
    pub fn previous(&self) -> Self {
        if self.0 == 0 {
            return Self(0);
        }
        Self::le(self.0 - 1)
    }

    /// Return the next palindromic number.
    ///
    /// **NOTE:** Highest return-value is [`Self::MAX`].
    pub fn next(&self) -> Self {
        if self.0 >= Self::MAX {
            return Self(Self::MAX);
        }
        Self::ge(self.0 + 1)
    }

    /// Return the first palindromic number that is less than or equal to `x`.
    pub fn le(x: u64) -> Self {
        if x.is_palindrome() {
            return Palindrome(x);
        }

        let (mut digits, mut length) = Self::digits_and_length(x);
        let half_length = length.div_ceil(2); // As in amount of digits.
        let mut fh_idx = half_length - 1;
        let mut sh_idx = half_length;
        if length % 2 == 1 {
            sh_idx -= 1; // We want center value of uneven number.
        }

        let mut skip = 0;
        loop {
            // 100 -> 99
            // 372 -> 363
            // 4847 -> 4774
            // 4003 -> 3993
            if digits[fh_idx] < digits[sh_idx] {
                return Self::construct_palindrome(length, &digits[..half_length]);
            }
            if digits[fh_idx] > digits[sh_idx] {
                // First try to downgrade center value, if it's 0, set to 9 and continue.
                // Once non-0 value found, -- it.
                let center_idx = half_length - 1; // Center idx.
                for i in 0..half_length {
                    if digits[center_idx - i] == 0 {
                        digits[center_idx - i] = 9;
                        continue;
                    }
                    digits[center_idx - i] -= 1;
                    // EDGE CASE: 100 -> 99 (length of first half digits CHANGES).
                    // EDGE CASE: 10 -> 9 (length of first half digits DOESN'T CHANGE).
                    if center_idx - i == 0 && digits[center_idx - i] == 0 {
                        digits[center_idx - i] = 9;
                        if length % 2 == 1 {
                            skip += 1;
                        }
                        length -= 1; // Length always decreases by 1.
                    }
                    break;
                }
                return Self::construct_palindrome(length, &digits[skip..half_length]);
            }

            fh_idx -= 1;
            sh_idx += 1;
        }
    }

    /// Return the first palindromic number that is greater than or equal to `x`.
    ///
    /// **ATTENTION:** Any value above [`Self::MAX`] will return [`Self::MAX`].
    pub fn ge(x: u64) -> Self {
        if x >= Self::MAX {
            return Self(Self::MAX);
        }

        if x.is_palindrome() {
            return Palindrome(x);
        }

        let (mut digits, length) = Self::digits_and_length(x);
        let half_length = length.div_ceil(2); // As in amount of digits.
        let mut fh_idx = half_length - 1;
        let mut sh_idx = half_length;
        if length % 2 == 1 {
            fh_idx -= 1; // We don't want center value of uneven number.
        }

        loop {
            if digits[fh_idx] > digits[sh_idx] {
                return Self::construct_palindrome(length, &digits[..half_length]);
            }
            if digits[fh_idx] < digits[sh_idx] {
                // First try to upgrade center value, if it's 9, set to 0 and continue.
                // Once non-9 value found, ++ it. 999 is palindrome and can't happen.
                let center_idx = half_length - 1; // Center idx.
                for i in 0..half_length {
                    if digits[center_idx - i] == 9 {
                        digits[center_idx - i] = 0;
                        continue;
                    }
                    digits[center_idx - i] += 1;
                    break;
                }
                return Self::construct_palindrome(length, &digits[..half_length]);
            }

            fh_idx -= 1;
            sh_idx += 1;
        }
    }
}

impl Display for Palindrome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Palindrome> for u64 {
    fn from(value: Palindrome) -> Self {
        value.0
    }
}

impl PartialEq<u64> for Palindrome {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Palindrome> for u64 {
    fn eq(&self, other: &Palindrome) -> bool {
        *self == other.0
    }
}

impl PartialOrd<u64> for Palindrome {
    fn ge(&self, other: &u64) -> bool {
        self.0 >= *other
    }

    fn gt(&self, other: &u64) -> bool {
        self.0 > *other
    }

    fn le(&self, other: &u64) -> bool {
        self.0 <= *other
    }

    fn lt(&self, other: &u64) -> bool {
        self.0 < *other
    }

    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Palindrome> for u64 {
    fn ge(&self, other: &Palindrome) -> bool {
        self >= other
    }

    fn gt(&self, other: &Palindrome) -> bool {
        self > other
    }

    fn le(&self, other: &Palindrome) -> bool {
        self <= other
    }

    fn lt(&self, other: &Palindrome) -> bool {
        self < other
    }

    fn partial_cmp(&self, other: &Palindrome) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl Add<u64> for Palindrome {
    type Output = u64;

    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        self.0 + rhs
    }
}

forward_ref_binop!(impl Add, add for Palindrome, u64);

impl Add<Palindrome> for u64 {
    type Output = u64;

    #[inline]
    fn add(self, rhs: Palindrome) -> Self::Output {
        self + rhs.0
    }
}

forward_ref_binop!(impl Add, add for u64, Palindrome);

impl AddAssign<Palindrome> for u64 {
    #[inline]
    fn add_assign(&mut self, rhs: Palindrome) {
        *self += rhs.0;
    }
}

forward_ref_op_assign!(impl AddAssign, add_assign for u64, Palindrome);

impl Div<u64> for Palindrome {
    type Output = u64;

    #[inline]
    fn div(self, rhs: u64) -> Self::Output {
        self.0 / rhs
    }
}

forward_ref_binop!(impl Div, div for Palindrome, u64);

impl Div<Palindrome> for u64 {
    type Output = u64;

    #[inline]
    fn div(self, rhs: Palindrome) -> Self::Output {
        self / rhs.0
    }
}

forward_ref_binop!(impl Div, div for u64, Palindrome);

impl DivAssign<Palindrome> for u64 {
    #[inline]
    fn div_assign(&mut self, rhs: Palindrome) {
        *self /= rhs.0
    }
}

forward_ref_op_assign!(impl DivAssign, div_assign for u64, Palindrome);

impl Mul<u64> for Palindrome {
    type Output = u64;

    #[inline]
    fn mul(self, rhs: u64) -> Self::Output {
        self.0 * rhs
    }
}

forward_ref_binop!(impl Mul, mul for Palindrome, u64);

impl Mul<Palindrome> for u64 {
    type Output = u64;

    #[inline]
    fn mul(self, rhs: Palindrome) -> Self::Output {
        self * rhs.0
    }
}

forward_ref_binop!(impl Mul, mul for u64, Palindrome);

impl MulAssign<Palindrome> for u64 {
    #[inline]
    fn mul_assign(&mut self, rhs: Palindrome) {
        *self *= rhs.0
    }
}

forward_ref_op_assign!(impl MulAssign, mul_assign for u64, Palindrome);

impl Rem<u64> for Palindrome {
    type Output = u64;

    #[inline]
    fn rem(self, rhs: u64) -> Self::Output {
        self.0 % rhs
    }
}

forward_ref_binop!(impl Rem, rem for Palindrome, u64);

impl Rem<Palindrome> for u64 {
    type Output = u64;

    #[inline]
    fn rem(self, rhs: Palindrome) -> Self::Output {
        self % rhs.0
    }
}

forward_ref_binop!(impl Rem, rem for u64, Palindrome);

impl RemAssign<Palindrome> for u64 {
    #[inline]
    fn rem_assign(&mut self, rhs: Palindrome) {
        *self %= rhs.0
    }
}

forward_ref_op_assign!(impl RemAssign, rem_assign for u64, Palindrome);

impl Sub<u64> for Palindrome {
    type Output = u64;

    #[inline]
    fn sub(self, rhs: u64) -> Self::Output {
        self.0 - rhs
    }
}

forward_ref_binop!(impl Sub, sub for Palindrome, u64);

impl Sub<Palindrome> for u64 {
    type Output = u64;

    #[inline]
    fn sub(self, rhs: Palindrome) -> Self::Output {
        self - rhs.0
    }
}

forward_ref_binop!(impl Sub, sub for u64, Palindrome);

impl SubAssign<Palindrome> for u64 {
    #[inline]
    fn sub_assign(&mut self, rhs: Palindrome) {
        *self -= rhs.0
    }
}

forward_ref_op_assign!(impl SubAssign, sub_assign for u64, Palindrome);

pub struct PalindromeIter {
    from: Palindrome,
    to: Palindrome,
    // Hacky solution, but can't be bothered to do it "properly".
    to_n_palindromes: usize,
    n: usize, // Current idx.
}

impl PalindromeIter {
    /// Iterate over all palindromes in the range `from..to`.
    ///
    /// **NOTE:** [`std::iter::Step`] is currently nightly/experimental,
    /// so this will have to do for now.
    pub fn from(from: Palindrome, to: Palindrome) -> Self {
        Self {
            from: Palindrome::ge(from.into()),
            to,
            to_n_palindromes: usize::MAX,
            n: 0,
        }
    }

    /// Iterate over all palindromes in the range `from..to`.
    pub fn from_u64(from: u64, to: u64) -> Self {
        Self {
            from: Palindrome::ge(from),
            // If it's not a palindrome, then we want to include the previous palindrome.
            to: Palindrome(to),
            to_n_palindromes: usize::MAX,
            n: 0,
        }
    }

    /// An iterator over the first `n` palindromes.
    ///
    /// **ATTENTION:** Panics if last palindrome would be larger than [`Palindrome::MAX`]
    pub fn first_n_palindromes(n: usize) -> Self {
        Self::first_n_palindromes_from(n, Palindrome(0))
    }

    /// An iterator over the first `n` palindromes from the first palindrome `from`.
    ///
    /// **ATTENTION:** Panics if last palindrome would be larger than [`Palindrome::MAX`].
    pub fn first_n_palindromes_from(n: usize, from: Palindrome) -> Self {
        // Length of 0..from
        let len_from_0 = Self::len_from_0(from.into());
        assert!(
            n + len_from_0 <= 11844674406,
            "A u64 value can't hold Palindromes any larger than the 11844674406th palindrome."
        );

        Self {
            from,
            to: Palindrome(u64::MAX),
            to_n_palindromes: n,
            n: 0,
        }
    }

    /// Calculate the length of [`Self`].
    ///
    /// **NOTE:** This function is constant time and much faster than [`Self::count`] for any non-trivial range.
    pub fn len(&self) -> usize {
        if self.to_n_palindromes < usize::MAX {
            return self.to_n_palindromes as usize;
        }

        // Calculate length from 0..self.from
        let over_counted = Self::len_from_0(self.from.into());

        // Calculate length from 0..self.to
        let over_count = Self::len_from_0(self.to.into());

        return over_count - over_counted;
    }

    // Doesn't include `to`.
    fn len_from_0(to: u64) -> usize {
        if to == 0 {
            return 0;
        }

        let (digits, length) = Palindrome::digits_and_length(to);
        let half_length = length.div_ceil(2);
        let front_part = &digits[0..half_length];

        let mut count = Self::palindromes_to_n_digits(length as u8) as isize;
        let mut front_part_as_num = 0isize;
        let mut to_subtract = 1isize;
        for x in front_part.iter() {
            to_subtract *= 10;
            front_part_as_num *= 10;
            front_part_as_num += *x as isize;
        }
        count += front_part_as_num - to_subtract;

        // If second half of the number is higher than first half, +1.
        let (mut i, mut j) = (half_length, half_length + 1);
        if length % 2 == 1 {
            i -= 1; // Don't want to compare center value of uneven digit number.
        }
        // Find the first digits from center and out that differ.
        while i > 0 && digits[i - 1] == digits[j - 1] {
            i -= 1;
            j += 1;
        }
        if i > 0 && digits[i - 1] < digits[j - 1] {
            count += 1; // Second half is larger, so ++ that bi***.
        }

        return count as usize;
    }

    fn palindromes_to_n_digits(n: u8) -> usize {
        if n == 0 {
            return 0;
        }

        let length = if n % 2 == 0 {
            2 * 10usize.pow(n as u32 / 2) - 1
        } else {
            11 * 10usize.pow(n as u32 / 2) - 1
        };

        return length;
    }
}

impl Iterator for PalindromeIter {
    type Item = Palindrome;

    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.from;

        if return_value < self.to && self.n < self.to_n_palindromes {
            self.from = self.from.next();
            self.n += 1;
            return Some(return_value);
        } else {
            return None;
        }
    }
}

pub trait IsPalindrome {
    /// Return whether `self` is a palindrome.
    fn is_palindrome(&self) -> bool;
}

impl IsPalindrome for u64 {
    fn is_palindrome(&self) -> bool {
        let mut x = *self;
        if x % 10 == 0 && x != 0 {
            return false;
        }

        let mut right_half = 0;
        while x > right_half {
            right_half = right_half * 10 + x % 10;
            x /= 10;
        }

        return x == right_half || x == right_half / 10;
    }
}

impl IsPalindrome for u32 {
    fn is_palindrome(&self) -> bool {
        (*self as u64).is_palindrome()
    }
}

impl IsPalindrome for u16 {
    fn is_palindrome(&self) -> bool {
        (*self as u64).is_palindrome()
    }
}

impl IsPalindrome for u8 {
    fn is_palindrome(&self) -> bool {
        (*self as u64).is_palindrome()
    }
}

impl IsPalindrome for Palindrome {
    fn is_palindrome(&self) -> bool {
        self.0.is_palindrome()
    }
}

#[cfg(test)]
mod tests {
    use crate::PalindromeIter;

    use super::Palindrome;

    #[test]
    fn test_palindrome_closest() {
        assert_eq!(11, Palindrome::closest(10));
        assert_eq!(38783, Palindrome::closest(38794));
        assert_eq!(38783, Palindrome::closest(38832));
        assert_eq!(38883, Palindrome::closest(38833));
        assert_eq!(943858349, Palindrome::closest(943854534));
    }

    #[test]
    fn test_construct_palindrome() {
        assert_eq!(34543, Palindrome::construct_palindrome(5, &vec![3, 4, 5]));
        assert_eq!(345543, Palindrome::construct_palindrome(6, &vec![3, 4, 5]));
        assert_eq!(0, Palindrome::construct_palindrome(1, &vec![0]));
        assert_eq!(0, Palindrome::construct_palindrome(2, &vec![0]));
        assert_eq!(
            1710171,
            Palindrome::construct_palindrome(7, &vec![1, 7, 1, 0])
        );
        assert_eq!(
            17100171,
            Palindrome::construct_palindrome(8, &vec![1, 7, 1, 0])
        );
    }

    #[test]
    #[should_panic]
    fn test_construct_palindrome_panic_on_too_short_length() {
        assert_eq!(34543, Palindrome::construct_palindrome(4, &vec![3, 4, 5]));
    }

    #[test]
    #[should_panic]
    fn test_construct_palindrome_panic_on_too_big_length() {
        assert_eq!(34543, Palindrome::construct_palindrome(7, &vec![3, 4, 5]));
    }

    #[test]
    fn test_palindrome_previous() {
        let pal = Palindrome(22);
        assert_eq!(11, pal.previous());
        let pal = Palindrome(998899);
        assert_eq!(997799, pal.previous());
        let pal = Palindrome(212);
        assert_eq!(202, pal.previous());
        let pal = Palindrome(202);
        assert_eq!(191, pal.previous());
        let pal = Palindrome(191);
        assert_eq!(181, pal.previous());
        let pal = Palindrome(1991);
        assert_eq!(1881, pal.previous());
        let pal = Palindrome(100001);
        assert_eq!(99999, pal.previous());
        let pal = Palindrome(1001);
        assert_eq!(999, pal.previous())
    }

    #[test]
    fn test_palindrome_next() {
        let pal = Palindrome(22);
        assert_eq!(33, pal.next());
        let pal = Palindrome(998899);
        assert_eq!(999999, pal.next());
        let pal = Palindrome(999999);
        assert_eq!(1000001, pal.next());
        let pal = Palindrome(212);
        assert_eq!(222, pal.next());
        let pal = Palindrome(191);
        assert_eq!(202, pal.next());
        let pal = Palindrome(181);
        assert_eq!(191, pal.next());
        let pal = Palindrome(1881);
        assert_eq!(1991, pal.next());
    }

    #[test]
    fn test_palindrome_le() {
        assert_eq!(9, Palindrome::le(10));
        assert_eq!(11, Palindrome::le(11));
        assert_eq!(11, Palindrome::le(19));
        assert_eq!(99, Palindrome::le(100));
        assert_eq!(997799, Palindrome::le(998001));
        assert_eq!(202, Palindrome::le(209));
        assert_eq!(191, Palindrome::le(201));
        assert_eq!(181, Palindrome::le(190));
        assert_eq!(1881, Palindrome::le(1990));
        assert_eq!(99999, Palindrome::le(100000));
        assert_eq!(999, Palindrome::le(1000));
        assert_eq!(34543, Palindrome::le(34550));
    }

    #[test]
    fn test_palindrome_ge() {
        assert_eq!(11, Palindrome::ge(10));
        assert_eq!(11, Palindrome::ge(11));
        assert_eq!(22, Palindrome::ge(19));
        assert_eq!(101, Palindrome::ge(100));
        assert_eq!(998899, Palindrome::ge(998001));
        assert_eq!(212, Palindrome::ge(209));
        assert_eq!(202, Palindrome::ge(199));
        assert_eq!(191, Palindrome::ge(190));
        assert_eq!(1991, Palindrome::ge(1990));
        assert_eq!(34543, Palindrome::ge(34504));
    }

    #[test]
    fn test_palindromeiter_first_n_palindromes() {
        // First test.
        let n = 912;
        let pal_iter = PalindromeIter::first_n_palindromes(n);
        assert_eq!(n, pal_iter.len());
        let mut count = 0;
        for _ in pal_iter {
            count += 1;
        }
        assert_eq!(n, count);

        // Second test.
        let n = 0;
        let pal_iter = PalindromeIter::first_n_palindromes(n);
        assert_eq!(n, pal_iter.len());
        let mut count = 0;
        for _ in pal_iter {
            count += 1;
        }
        assert_eq!(n, count);

        // Third test.
        let n = 1;
        let pal_iter = PalindromeIter::first_n_palindromes(n);
        assert_eq!(n, pal_iter.len());
        let mut count = 0;
        for _ in pal_iter {
            count += 1;
        }
        assert_eq!(n, count);

        // Fourth test.
        let n = 32903;
        let pal_iter = PalindromeIter::first_n_palindromes(n);
        assert_eq!(n, pal_iter.len());
        let mut count = 0;
        for _ in pal_iter {
            count += 1;
        }
        assert_eq!(n, count);
    }

    #[test]
    fn test_palindromeiter_first_n_palindromes_from() {
        // First test.
        let n = 912;
        let pal_iter = PalindromeIter::first_n_palindromes_from(n, Palindrome::le(9));
        assert_eq!(n, pal_iter.len());
        let mut count = 0;
        for _ in pal_iter {
            count += 1;
        }
        assert_eq!(n, count);

        // Second test.
        let n = 0;
        let pal_iter = PalindromeIter::first_n_palindromes_from(n, Palindrome::closest(38743));
        assert_eq!(n, pal_iter.len());
        let mut count = 0;
        for _ in pal_iter {
            count += 1;
        }
        assert_eq!(n, count);

        // Third test.
        let n = 1;
        let pal_iter = PalindromeIter::first_n_palindromes_from(n, Palindrome::ge(98734));
        assert_eq!(n, pal_iter.len());
        let mut count = 0;
        for _ in pal_iter {
            count += 1;
        }
        assert_eq!(n, count);

        // Fourth test.
        let n = 32903;
        let pal_iter = PalindromeIter::first_n_palindromes_from(n, Palindrome::le(2222));
        assert_eq!(n, pal_iter.len());
        let mut count = 0;
        for _ in pal_iter {
            count += 1;
        }
        assert_eq!(n, count);
    }

    #[test]
    fn test_palindromeiter_len() {
        // 10.
        let pal_iter = PalindromeIter::from_u64(0, 10);
        assert_eq!(pal_iter.len(), pal_iter.count());
        let pal_iter = PalindromeIter::from_u64(2, 10);
        assert_eq!(pal_iter.len(), pal_iter.count());
        let pal_iter = PalindromeIter::from_u64(3, 11);
        assert_eq!(pal_iter.len(), pal_iter.count());
        // 100.
        let pal_iter = PalindromeIter::from_u64(0, 100);
        assert_eq!(pal_iter.len(), pal_iter.count());
        let pal_iter = PalindromeIter::from_u64(45, 100);
        assert_eq!(pal_iter.len(), pal_iter.count());
        let pal_iter = PalindromeIter::from_u64(55, 100);
        assert_eq!(pal_iter.len(), pal_iter.count());
        let pal_iter = PalindromeIter::from_u64(53, 101);
        assert_eq!(pal_iter.len(), pal_iter.count());
        // 1000.
        let pal_iter = PalindromeIter::from_u64(0, 1000);
        assert_eq!(pal_iter.len(), pal_iter.count());
        let pal_iter = PalindromeIter::from_u64(34, 1000);
        assert_eq!(pal_iter.len(), pal_iter.count());
        let pal_iter = PalindromeIter::from_u64(0, 1000);
        assert_eq!(pal_iter.len(), pal_iter.count());
        // 10_000.
        let pal_iter = PalindromeIter::from_u64(0, 10_000);
        assert_eq!(pal_iter.len(), pal_iter.count());
        let pal_iter = PalindromeIter::from_u64(0, 10_000);
        assert_eq!(pal_iter.len(), pal_iter.count());
        let pal_iter = PalindromeIter::from_u64(0, 10_000);
        assert_eq!(pal_iter.len(), pal_iter.count());
        // Edge case.
        let pal_iter = PalindromeIter::from_u64(0, 668);
        assert_eq!(pal_iter.len(), pal_iter.count());
    }
}
