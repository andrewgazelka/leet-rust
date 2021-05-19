use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complement = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            let other_index = complement.get(x);
            if let Some(ii) = other_index { return vec![*ii as i32, i as i32]; }
            let key = target - x;
            complement.insert(key, i);
        }
        unreachable!()
    }
}
