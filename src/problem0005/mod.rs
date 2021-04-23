use std::cmp::min;

use crate::Solution;

struct Longest {
    left: usize, // left index (inclusive)
    right: usize, // right index (inclusive)
    size: usize, // total size of string
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {

        // if the string is 1 length or shorter it is automatically a palindrome.
        // this is also important because a lot of the code assumes the string is at least length 1
        if s.len() <= 1 {
            return s;
        }

        // so we can grab a char by its index
        let chars: Vec<_> = s.chars().collect();

        // tracks the longest string. Initially 0-sized. Since we know the string has more than one character,
        // it must get a valid value
        let mut longest: Longest = Longest { left: 0, right: 0, size: 0 };

        // the max index of the string
        let max_i = s.len() - 1;

        for i in 0..s.len() {

            // odd-length palindromes (has one character as a center)
            let pivot = i;
            let max_one_side = min(max_i - i, i);
            let mut one_side_len = 0;
            for j in 1..=max_one_side {
                let a = chars[pivot - j];
                let b = chars[pivot + j];
                if a != b {
                    break;
                }
                one_side_len += 1;
            }
            let size = one_side_len * 2 + 1;
            if size > longest.size {
                longest = Longest {
                    left: pivot - one_side_len,
                    right: pivot + one_side_len,
                    size,
                }
            }

            // even-length palindromes. This is i32 instead of usize as there are some cases where max_i - i - 1 will
            // be less than 0 and this is the easiest way to handle that
            let max_one_side: i32 = min((max_i - i) as i32 - 1, i as i32);

            // in the odd-length counting one-sided length did not count center, but this one does
            // for example in odd-length a one_side_len of 0 would be one character, where this would
            // be ""
            let mut one_side_len = 0;
            for j in 0..=max_one_side {
                let j = j as usize;
                let a = chars[pivot - j];
                let b = chars[pivot + j + 1];
                if a != b {
                    break;
                }
                one_side_len += 1;
            }
            let size = one_side_len * 2;
            if size > longest.size {
                longest = Longest {
                    left: pivot - one_side_len + 1, // +1 does not make sense for OSL = 0, but consider all other OSLs
                    right: pivot + one_side_len,
                    size,
                }
            }
        }
        let char_res = &chars[longest.left..=longest.right];
        char_res.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let palind = Solution::longest_palindrome("abccbracecar".to_string());
        assert_eq!(palind, "racecar".to_string());
    }
}
