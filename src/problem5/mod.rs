use std::cmp::min;

use crate::Solution;
struct Longest {
    left: usize,
    right: usize,
    size: usize,
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
        let chars: Vec<_> = s.chars().collect();
        let len = s.len();
        let mut longest: Longest = Longest { left: 0, right: 0, size: 0 };
        let max_i = s.len() - 1;
        for i in 0..len {
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

            let max_one_side: i32 = min((max_i - i) as i32 - 1, i as i32);

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
                    left: pivot - one_side_len + 1,
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
        println!("palind {}", palind)
    }
}
