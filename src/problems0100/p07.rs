use crate::Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        Self::reverse_opt(x).unwrap_or(0)
    }

    #[inline]
    pub fn reverse_opt(mut x: i32) -> Option<i32> {
        let mut result = 0 as i32;

        while x != 0 {
            println!("{}", result);
            result = result.checked_mul(10)?;
            result = result.checked_add(x % 10)?;
            x /= 10;
        }

        Option::from(result)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
