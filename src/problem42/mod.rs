use std::cmp::{max};

use crate::Solution;

impl Solution {
    // heap

    /*


    x   x x
    xxx x x
     */
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();


        // we cannot trap any water
        if len <= 2 {
            return 0;
        }


        let mut total_water = 0;

        let mut left_max_height = 0;
        let mut right_max_height = 0;

        let mut left_idx = 0;
        let mut right_idx = len - 1;

        while left_idx < right_idx {
            let left_height = height[left_idx];
            if left_height > left_max_height {
                left_max_height = left_height;

                while right_idx > left_idx {
                    let right_height = height[right_idx];
                    right_max_height = max(right_height, right_max_height);
                    if right_max_height > left_max_height {
                        break;
                    } else {
                        let fill = right_max_height - right_height;
                        total_water += fill;
                    }
                    right_idx -= 1;
                }
            } else {
                total_water += left_max_height - left_height;
            }

            left_idx += 1;
        }

        total_water
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
