//! 412. Fizz Buzz

use crate::Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut vec = Vec::with_capacity(n);
        for i in 1..=n {
            let divis_3 = i % 3 == 0;
            let divis_5 = i % 5 == 0;

            let res = match (divis_3, divis_5) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                (false, false) => i.to_string()
            };

            vec.push(res);
        }
        vec
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let solve = Solution::fizz_buzz;
        assert_eq!(solve(15), vec!["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"]);
    }
}
