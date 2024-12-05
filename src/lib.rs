#[derive(Debug, Clone, Copy)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Construct a palindrome from the first half of a digit and a provided length.
    /// Will panic if 'length' isn't 2x or 2x - 1 the size of 'digits_half.len()'.
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

    /// Return the first palindromic number that is smaller.
    /// NOTE: Lowest return-value is '0'.
    pub fn decrement(&self) -> Self {
        if self.0 == 0 {
            return Self(0);
        }
        Self::le(self.0 - 1)
    }

    /// Return the first palindromic number that is larger.
    /// NOTE: Highest return-value is the first palindromic number less than 'u64::MAX'.
    pub fn increment(&self) -> Self {
        // Largest possible palindrome to fit in u64.
        if self.0 >= 18446744066044764481 {
            return Self(self.0);
        }
        Self::ge(self.0 + 1)
    }

    /// Return whether 'x' is a palindrome. Same as 'is_palindromic()'.
    pub fn is_palindrome(x: u64) -> bool {
        Self::is_palindromic(x)
    }

    /// Return whether 'x' is palindromic. Same as 'is_palindrome()'.
    pub fn is_palindromic(mut x: u64) -> bool {
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

    /// Return the first palindromic number that is less than or equal to 'x'.
    pub fn le(x: u64) -> Self {
        if Self::is_palindromic(x) {
            return Palindrome(x);
        }

        let (digits, mut length) = Self::digits_and_length(x);
        let first_half_length = length.div_ceil(2);

        // We want to promote numbers that would create a palindrome less than x.
        // Ex: x=1451 would create the palindrome 1441, which is less than x.
        let half_digits = &digits[..first_half_length];
        let mut new_digits = digits.clone();
        let mut front_zeroes_removed = 0;
        // We go about it by going from the middle to the end of the palindrome,
        // which is the same as going from the middle to the start of the 'digits'.
        // Surely nothing can go wrong...
        'outer: for (rev_idx, x) in half_digits.iter().rev().enumerate() {
            // We then compare that first half of 'digits' with the corresponding second half.
            if x > &digits[length - first_half_length + rev_idx] {
                // If any digit is greater than its corresponding digit,
                // then we start over from the middle, demoting relevant digits.
                for i in 0..first_half_length {
                    let left_side_idx = first_half_length - 1 - i;

                    // We can't exactly demote a number to less than 0.
                    if new_digits[left_side_idx] == 0 {
                        new_digits[left_side_idx] = 9;
                        continue; // We're not done demoting...
                    }

                    // In case we change the first digit to 0.
                    if left_side_idx == 0 && new_digits[left_side_idx] == 1 {
                        front_zeroes_removed += 1;
                        length -= 1;
                        // Edge case: when a number like 10 or 1000 with an even
                        // length is encountered, as we don't want to remove
                        // the relevant half_digit, but just reduce the length and
                        // set the front digit to 9.
                        if length % 2 == 1 {
                            front_zeroes_removed -= 1;
                            new_digits[left_side_idx] = 9;
                        }
                        break;
                    }

                    new_digits[left_side_idx] -= 1;
                    break 'outer;
                }
            }
        }

        let palindrome = Self::construct_palindrome(
            length,
            &new_digits[front_zeroes_removed..first_half_length],
        );
        palindrome
    }

    /// Return the first palindromic number that is greater than or equal to 'x'.
    pub fn ge(x: u64) -> Self {
        // Largest possible palindrome to fit in u64.
        // FIXME: This is returning a palindromic number LESS than 'x'.
        if x > 18446744066044764481 {
            return Self(18446744066044764481);
        }

        if Self::is_palindromic(x) {
            return Palindrome(x);
        }

        let (digits, length) = Self::digits_and_length(x);
        let first_half_length = length.div_ceil(2);

        // We want to promote numbers that would create a palindrome less than x.
        // Ex: x=1451 would create the palindrome 1441, which is less than x.
        let half_digits = &digits[..first_half_length];
        let mut new_digits = digits.clone();
        let mut front_ones_added = 0;
        // We go about it by going from the middle to the end of the palindrome,
        // which is the same as going from the middle to the start of the 'digits'.
        // Surely nothing can go wrong...
        for (rev_idx, x) in half_digits.iter().rev().enumerate() {
            // We then compare that first half of 'digits' with the corresponding second half.
            if x < &digits[length - first_half_length + rev_idx] {
                // If any digit is less than its corresponding digit,
                // then we start over from the middle, promoting relevant digits.
                for i in 0..first_half_length {
                    let left_side_idx = first_half_length - 1 - i;

                    // We can't exactly promote a number to 10.
                    if new_digits[left_side_idx] == 9 {
                        new_digits[left_side_idx] = 0;
                        // In case we changed the first digit.
                        if left_side_idx == 0 {
                            new_digits.insert(0, 1);
                            front_ones_added += 1;
                        }
                        continue; // We're not done promoting...
                    }
                    new_digits[left_side_idx] += 1;
                    break;
                }
            }
        }

        let palindrome = Self::construct_palindrome(
            length + front_ones_added,
            &new_digits[..first_half_length + front_ones_added],
        );
        palindrome
    }
}

impl From<Palindrome> for u64 {
    fn from(value: Palindrome) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::Palindrome;

    #[test]
    fn test_construct_palindrome() {
        assert_eq!(34543, Palindrome::construct_palindrome(5, &vec![3, 4, 5]).0);
        assert_eq!(
            345543,
            Palindrome::construct_palindrome(6, &vec![3, 4, 5]).0
        );
        assert_eq!(0, Palindrome::construct_palindrome(1, &vec![0]).0);
        assert_eq!(0, Palindrome::construct_palindrome(2, &vec![0]).0);
        assert_eq!(
            1710171,
            Palindrome::construct_palindrome(7, &vec![1, 7, 1, 0]).0
        );
        assert_eq!(
            17100171,
            Palindrome::construct_palindrome(8, &vec![1, 7, 1, 0]).0
        );
    }

    #[test]
    #[should_panic]
    fn test_construct_palindrome_panic_on_too_short_length() {
        assert_eq!(34543, Palindrome::construct_palindrome(4, &vec![3, 4, 5]).0);
    }

    #[test]
    #[should_panic]
    fn test_construct_palindrome_panic_on_too_big_length() {
        assert_eq!(34543, Palindrome::construct_palindrome(7, &vec![3, 4, 5]).0);
    }

    #[test]
    fn test_palindrome_decrement() {
        let pal = Palindrome(22);
        assert_eq!(11, pal.decrement().0);
        let pal = Palindrome(998899);
        assert_eq!(997799, pal.decrement().0);
        let pal = Palindrome(212);
        assert_eq!(202, pal.decrement().0);
        let pal = Palindrome(202);
        assert_eq!(191, pal.decrement().0);
        let pal = Palindrome(191);
        assert_eq!(181, pal.decrement().0);
        let pal = Palindrome(1991);
        assert_eq!(1881, pal.decrement().0);
        let pal = Palindrome(100001);
        assert_eq!(99999, pal.decrement().0);
        let pal = Palindrome(1001);
        assert_eq!(999, pal.decrement().0)
    }

    #[test]
    fn test_palindrome_increment() {
        let pal = Palindrome(22);
        assert_eq!(33, pal.increment().0);
        let pal = Palindrome(998899);
        assert_eq!(999999, pal.increment().0);
        let pal = Palindrome(999999);
        assert_eq!(1000001, pal.increment().0);
        let pal = Palindrome(212);
        assert_eq!(222, pal.increment().0);
        let pal = Palindrome(191);
        assert_eq!(202, pal.increment().0);
        let pal = Palindrome(181);
        assert_eq!(191, pal.increment().0);
        let pal = Palindrome(1881);
        assert_eq!(1991, pal.increment().0);
    }

    #[test]
    fn test_palindrome_le() {
        assert_eq!(9, Palindrome::le(10).0);
        assert_eq!(11, Palindrome::le(11).0);
        assert_eq!(11, Palindrome::le(19).0);
        assert_eq!(99, Palindrome::le(100).0);
        assert_eq!(997799, Palindrome::le(998001).0);
        assert_eq!(202, Palindrome::le(209).0);
        assert_eq!(191, Palindrome::le(201).0);
        assert_eq!(181, Palindrome::le(190).0);
        assert_eq!(1881, Palindrome::le(1990).0);
        assert_eq!(99999, Palindrome::le(100000).0);
        assert_eq!(999, Palindrome::le(1000).0)
    }

    #[test]
    fn test_palindrome_ge() {
        assert_eq!(11, Palindrome::ge(10).0);
        assert_eq!(11, Palindrome::ge(11).0);
        assert_eq!(22, Palindrome::ge(19).0);
        assert_eq!(101, Palindrome::ge(100).0);
        assert_eq!(998899, Palindrome::ge(998001).0);
        assert_eq!(212, Palindrome::ge(209).0);
        assert_eq!(202, Palindrome::ge(199).0);
        assert_eq!(191, Palindrome::ge(190).0);
        assert_eq!(1991, Palindrome::ge(1990).0);
    }
}
