use std::collections::BTreeSet;

use crate::Solution;

fn pop(set: &mut BTreeSet<i32>) -> Option<i32> {
    let ret = set.iter().next().cloned();
    if let Some(elem) = ret {
        set.remove(&elem);
    }
    ret
}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set: BTreeSet<i32> = nums.into_iter().collect();

        let mut longest_sequence = 0;

        while let Some(elem) = pop(&mut set) {
            let mut current_sequence = 1;
            let mut prev = elem - 1;
            while set.remove(&prev) {
                current_sequence += 1;
                prev -= 1;
            }
            let mut next = elem + 1;
            while set.remove(&next) {
                current_sequence += 1;
                next += 1;
            }
            if current_sequence > longest_sequence {
                longest_sequence = current_sequence;
            }
        }

        longest_sequence
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }
}
