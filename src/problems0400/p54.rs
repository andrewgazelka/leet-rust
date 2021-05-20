//! 4  sum 2

use std::collections::HashMap;

use crate::Solution;

fn multiply(map: &HashMap<i32, i32>, vec: &[i32]) -> HashMap<i32, i32> {
    let mut new_map = HashMap::new();
    for v_elem in vec {
        for (m_key, m_val) in map {
            let sum_key = v_elem + m_key;
            *new_map.entry(sum_key).or_insert(0) += m_val;
        }
    }
    new_map
}

impl Solution {
    /// precondition: nums1, nums2, nums3, nums4 length same
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 1);

        map = multiply(&map, &nums1);
        map = multiply(&map, &nums2);
        map = multiply(&map, &nums3);

        // can optimize here
        map = multiply(&map, &nums4);

        *map.get(&0).unwrap_or(&0)
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

        assert_eq!(solve(vec![1],vec![-1],vec![0],vec![1]), 0);
    }
}
