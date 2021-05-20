//! 4  sum 2

use std::collections::HashMap;

use crate::Solution;


impl Solution {
    /// precondition: nums1, nums2, nums3, nums4 length same
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        // inspiration from https://leetcode.com/problems/4sum-ii/discuss/442608/C%2B%2B-or-Fast-and-efficient

        let mut ab = HashMap::new();

        for a in nums1 {
            for b in &nums2 {
                *ab.entry(a+b).or_insert(0) += 1;
            }
        }

        let mut count = 0;

        for b in nums3 {
            for c in &nums4 {
                let need = -(b+c);
                count += ab.get(&need).unwrap_or(&0)
            }
        }

        count
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let solve = Solution::four_sum_count;

        assert_eq!(solve(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]), 2);
        assert_eq!(solve(vec![0], vec![0], vec![0], vec![0]), 1);

        assert_eq!(solve(vec![1], vec![-1], vec![0], vec![1]), 0);
    }
}
