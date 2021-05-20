//! 4  sum 2

use std::collections::HashMap;

use crate::Solution;

fn multiply(map: &HashMap<i32, i32>, vec: &[i32], new_map: &mut HashMap<i32, i32>) {
    for v_elem in vec {
        for (m_key, m_val) in map {
            let sum_key = v_elem + m_key;
            *new_map.entry(sum_key).or_insert(0) += m_val;
        }
    }
}

impl Solution {
    /// precondition: nums1, nums2, nums3, nums4 length same
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut base_map = HashMap::new();
        base_map.insert(0, 1);

        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        let mut map3 = HashMap::new();

        multiply(&base_map, &nums1, &mut map1);
        multiply(&map1, &nums2, &mut map2);
        multiply(&map2, &nums3, &mut map3);

        let mut count = 0;

        for elem in nums4 {
            let need = -elem;
            count += map3.get(&need).unwrap_or(&0)
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
