use std::cmp::max;
use std::collections::{HashMap};

use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0
        }
        let mut map = HashMap::with_capacity(s.len());
        let mut max_length = 0;
        let mut start_idx = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(start_i) = map.get(&c) {
                if *start_i > start_idx {
                    max_length = max(max_length, i - start_i);
                    start_idx = *start_i + 1;
                }
            }
            map.insert(c, i);
        }
        max((s.len() - 1) - start_idx, max_length) as i32
    }
}
