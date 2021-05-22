use std::cmp::{min};

use crate::Solution;

#[inline]
fn ival(character: char) -> usize {
    character as usize - 'a' as usize
}

// this would be i32::MAX but leetcode does not allow for some reason
const MAX: i32 = 2147483647;


impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut next_unique = [true; 26];
        let mut first_idx = [MAX; 26];

        for (s_idx, c) in s.chars().enumerate() {
            let c = ival(c);
            first_idx[c] = if next_unique[c] {
                s_idx as i32
            } else {
                MAX
            };

            next_unique[c] = false;
        }

        let mut min_idx = MAX;
        for fi in first_idx.iter().cloned() {
            min_idx = min(fi, min_idx);
        }

        if min_idx == MAX {
            -1
        } else {
            min_idx
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let solve = |s: &str| Solution::first_uniq_char(s.to_string());

        assert_eq!(solve("leetcode"), 0);
        assert_eq!(solve("loveleetcode"), 2);
        assert_eq!(solve("aabb"), -1);
        assert_eq!(solve(""), -1);
        assert_eq!(solve("a"), 0);
        assert_eq!(solve("aab"), 2);
        assert_eq!(solve("baa"), 0);
    }
}
