use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::Solution;

impl Solution {
    // heap
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let nums_over_zero: Vec<_> = nums.into_iter()
            .filter(|&x| x > 0) // only consider positive values
            .map(Reverse) // since we need a min heap
            .collect();

        let mut heap = BinaryHeap::from(nums_over_zero); // create min heap

        let mut expect = 1; // the number we expect to receive

        while let Some(Reverse(elem)) = heap.pop() { // take min elem O(log2(n)) for each pop
            if elem < expect {
                continue; // we have duplicate for example [1,2,2,2,3] would have multiple 2s
            }
            if expect != elem {
                return expect; // we don't have that element so this is the missing number
            }
            expect += 1;
        }

        expect // all numbers in `nums` are in a sequence... the missing number is one greater than the largest in nums (or 1)
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::first_missing_positive(vec![0, 2, 2, 1, 1]), 3);
    }
}
