use std::cmp::min;

use crate::Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        if len <= 1 {
            return 0;
        }

        let mut max_area = 0;

        let mut left_idx = 0_usize;
        let mut right_idx = len - 1;

        loop {
            let left_height = heights[left_idx] as usize;
            let right_height = heights[right_idx] as usize;
            let width = right_idx - left_idx;
            if width == 0 { break; }

            let height = min(left_height, right_height);

            let area = height * width;
            if area > max_area {
                max_area = area
            }

            let move_left_idx = left_height < right_height;

            if move_left_idx {
                left_idx += 1;
            } else {
                right_idx -= 1;
            }
        }


        max_area as i32
    }
}



#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
    fn it_works() {
        let solve = Solution::max_area;

        assert_eq!(solve(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(solve(vec![1,1]), 1);
        assert_eq!(solve(vec![4,3,2,1,4]), 16);
        assert_eq!(solve(vec![1,2,1]), 2);
    }
}
