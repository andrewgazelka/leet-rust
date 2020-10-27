use crate::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let str = x.to_string();
        // x x x    3 [0,1) [2,0]   ... 3/2 ... (3+1)/2
        // x x x x  4 [0,2) [2,0]  ... 3/2 ... (3+1)/2
        let len = str.len();

        let chars = str.as_bytes();

        for i in 0..len / 2 {
            if chars[i] != chars[len - i - 1] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_palindrome(1331), true);
        assert_eq!(Solution::is_palindrome(13331), true);
        assert_eq!(Solution::is_palindrome(135331), false);
    }
}
