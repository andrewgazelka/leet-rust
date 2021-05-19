use std::collections::{HashMap};

use crate::Solution;

impl Solution {

    /// inspiration from https://leetcode.com/problems/subarray-sum-equals-k/discuss/1210806/Cpp-Solution
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {

        // num -> count
        let mut map = HashMap::new();

        map.insert(0, 1);

        let mut total = 0;
        let mut pre_sum = 0;

        for num in nums {

            // the sum from the left side to the index the num is at
            pre_sum += num;

            // moving index to right result (normalized with k)
            let right_add = pre_sum - k;

            total += map.get(&right_add).unwrap_or(&0);

            // moving index to left result
            *map.entry(pre_sum).or_insert(0) += 1;
        }
        total
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1],  2));
        assert_eq!(2, Solution::subarray_sum(vec![1, 2, 3],  3));
    }
}
