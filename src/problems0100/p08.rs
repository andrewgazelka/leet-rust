// use crate::Solution;
// use std::num::{IntErrorKind};

// fn trim(s: String) -> String {
//     s.chars()
//         .skip_while(|x| x.is_ascii_whitespace())
//         .enumerate()
//         .take_while(|&(i, c) | match c {
//             '0'..='9' => true,
//             '+' | '-' => i == 0 as usize,
//             _ => false,
//         })
//         .map(|(_, c)| c)
//         .collect::<String>()
// }

// impl Solution {
//     pub fn my_atoi(s: String) -> i32 {
//         let trimmed = trim(s);
//         match trimmed.len() {
//             0 => 0,
//             1 => trimmed.parse().unwrap_or(0),
//             _ => match trimmed.parse::<i32>() {
//                 Ok(x) => x,
//                 Err(e) if e.kind() == &IntErrorKind::Overflow => i32::MAX,
//                 _ => i32::MIN,
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    // use crate::Solution;

    #[test]
    fn it_works() {
        // assert_eq!(Solution::my_atoi("42".to_string()), 42);
        // assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
        // assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        // assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        // assert_eq!(Solution::my_atoi("999999999999999999999999999".to_string()), i32::MAX);
        // assert_eq!(Solution::my_atoi("-999999999999999999999999999".to_string()), i32::MIN);
    }
}
