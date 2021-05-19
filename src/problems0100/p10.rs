use std::ops::{Index, IndexMut};

use crate::Solution;

fn matches(regex_char: char, real_char: char) -> bool {
    match regex_char {
        '.' => true,
        c => c == real_char
    }
}

struct Arr2D<T> {
    backing: Vec<T>,
    width: usize,
}

impl<T: Clone> Arr2D<T> {
    fn new(x: usize, y: usize, default: T) -> Arr2D<T> {
        Self {
            backing: vec![default; x * y],
            width: x,
        }
    }
}

impl<T> Index<(usize, usize)> for Arr2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let x = index.0;
        let y = index.1;
        let idx = x + y * self.width;
        &self.backing[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Arr2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let x = index.0;
        let y = index.1;
        let idx = x + y * self.width;
        &mut self.backing[idx]
    }
}

#[allow(unused_variables)]
impl Solution {
    /// support *, .
    /// match *entire* string
    pub fn is_match(string: String, pattern: String) -> bool {
        let string: Vec<_> = string.chars().collect();
        let pattern: Vec<_> = pattern.chars().collect();

        let str_len = string.len();
        let pattern_len = pattern.len();


        let mut dp = Arr2D::new(str_len + 1, pattern_len + 1, false);

        dp[(str_len, pattern_len)] = true;
        dp[(str_len - 1, pattern_len)] = true;

        for (s_idx, sc) in string.into_iter().enumerate().rev() {
            for (p_idx, &pc) in pattern.iter().enumerate().rev() {
                let char_match = matches(pc, sc);

                let asterisk = match pattern.get(p_idx + 1) {
                    None => false,
                    Some(&c) => c == '*',
                };

                dp[(s_idx, p_idx)] = if asterisk {
                    let valid_no_match = dp[(s_idx, p_idx + 2)];
                    let valid_match = char_match && dp[(s_idx + 1, p_idx)];

                    valid_no_match || valid_match
                } else {
                    char_match && dp[(s_idx + 1, p_idx + 1)]
                }
            }
        }

        dp[(0, 0)]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    fn is_match(a: &str, b: &str) -> bool {
        Solution::is_match(a.to_string(), b.to_string())
    }

    #[test]
    fn it_works() {
        assert!(!is_match("aa", "a"));
        // assert!(is_match("aa", "a*"));
        // assert!(is_match("ab", ".*"));
        // assert!(is_match("aab", "c*a*b"));
        // assert!(!is_match("mississippi", "mis*is*p*."));
    }
}
