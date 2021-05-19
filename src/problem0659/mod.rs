use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

use crate::Solution;

// order: close to 3 ...

struct LenStrat(usize);

impl Ord for LenStrat {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.0;
        let b = other.0;

        if a < 3 && b >= 3 {
            Ordering::Greater
        } else {
            a.partial_cmp(&b).unwrap()
        }
    }
}

impl PartialOrd for LenStrat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for LenStrat {}

impl PartialEq for LenStrat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}


impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut previous = nums[0];

        let mut sequence_on = 0;
        let mut seq_lens = BinaryHeap::from(vec![LenStrat(1)]);

        for grabbed in nums.into_iter().skip(1) {
            let dif = grabbed - previous;

            match dif {
                0 => {
                    // we cannot add to this sequence
                    if sequence_on == 0 {

                        // we must add another sequence
                        seq_lens.push(LenStrat(1));
                    } else {
                        sequence_on -= 1;
                    }
                }

                1 => {
                    // remove all the sequences we couldn't reach
                    for _ in 0..sequence_on {
                        let LenStrat(len) = seq_lens.pop().unwrap();

                        if len < 3 {
                            return false;
                        }
                    }

                    // add one to length
                    let iter = seq_lens.into_iter().map(|LenStrat(x)| LenStrat(x + 1));
                    seq_lens = BinaryHeap::from_iter(iter);

                    sequence_on = seq_lens.len() - 1;
                }

                _ => {
                    while let Some(LenStrat(elem)) = seq_lens.pop() {
                        if elem < 3 {
                            return false;
                        }
                    }

                    seq_lens.push(LenStrat(1));
                    sequence_on = 0;
                }
            }

            previous = grabbed;
        }

        seq_lens.into_iter().all(|LenStrat(len)| len >= 3)
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert!(!Solution::is_possible(vec![3, 4, 4, 5, 6, 7, 8, 9, 10, 11]));

        // let big_vec = vec![14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 20, 21, 21, 21, 22, 22, 22, 23, 23, 23, 24, 24, 24, 24, 25, 25, 25, 25, 26, 26,
        //                    26, 26, 27, 27, 27, 27, 28, 28, 28, 28, 29, 29, 29, 30, 30, 30,
        //                    31, 31, 31, 32, 32, 32, 33, 33, 33, 34, 34, 34, 35, 35, 35, 36, 36, 36, 37, 37, 37, 38, 38, 38, 39, 39, 39, 40, 40, 40, 41, 41, 41, 42, 42, 43, 43, 44, 44, 45, 45, 46, 46, 47, 47, 47, 48, 48, 48, 49, 49, 49, 50, 50, 50, 51, 51, 51, 52, 52, 52, 53, 53, 53, 54, 54, 54, 55, 55, 55, 56, 56, 56, 57, 57, 57, 58, 58, 58, 59, 59, 59, 60, 60, 60, 61, 61, 61, 62, 62, 62, 62, 63, 63, 63, 63, 64, 64, 64, 64, 65, 65, 65, 65, 65, 66, 66, 66, 66, 66, 67, 67, 67, 67, 67, 68, 68, 68, 68, 68, 68, 69, 69, 69, 69, 69, 69, 70, 70, 70, 70, 70, 70, 71, 71, 71, 71, 71, 71, 72, 72, 72, 72, 72, 72, 73, 73, 73, 73, 73, 73, 74, 74, 74, 74, 74, 74, 75, 75, 75, 75, 75, 75, 76, 76, 76, 76, 76, 76, 77, 77, 77, 77, 77, 77, 78, 78, 78, 78, 78, 78, 79, 79, 79, 79, 79, 79, 80, 80, 80, 80, 80, 80, 80, 81, 81, 81, 81, 81, 81, 81, 82, 82, 82, 82, 82, 82, 82, 83, 83, 83, 83, 83, 83, 83, 84, 84, 84, 84, 84, 84, 84, 85, 85, 85, 85, 85, 85, 85, 86, 86, 86, 86, 86, 86, 86, 86, 87, 87, 87, 87, 87, 87, 87, 87, 88, 88, 88, 88, 88, 88, 88, 88, 89, 89, 89, 89, 89, 89, 89, 89, 90, 90, 90, 90, 90, 90, 90, 90, 91, 91, 91, 91, 91, 91, 91, 92, 92, 92, 92, 92, 92, 92, 93, 93, 93, 93, 93, 93, 93, 94, 94, 94, 94, 94, 94, 95, 95, 95, 95, 95, 95, 96, 96, 96, 96, 96, 96, 97, 97, 97, 97, 97, 97, 98, 98, 98, 98, 98, 98, 99, 99, 99, 99, 99, 99, 100, 100, 100, 100, 100, 101, 101, 101, 101, 101, 102, 102, 102, 102, 102, 103, 103, 103, 103, 103, 104, 104, 104, 104, 104, 105, 105, 105, 105, 105, 106, 106, 106, 106, 106, 107, 107, 107, 107, 107, 108, 108, 108, 108, 108, 109, 109, 109, 109, 109, 110, 110, 110, 110, 110, 111, 111, 111, 111, 111, 112, 112, 112, 113, 113, 113, 114, 114, 114, 115, 115, 115, 116, 116, 116, 117, 117, 117, 118, 118, 118, 119, 119, 119, 120, 120, 120, 121, 121, 121, 122, 122, 122, 123, 123, 123, 124, 124, 124, 125, 125, 125, 126, 126, 126, 127, 127, 127, 128, 128, 128, 129, 129, 129, 130, 130, 130, 131, 131, 131, 132, 132, 132, 133, 133, 133, 134, 134, 135, 135, 136, 136, 137, 137, 138, 138, 139, 139, 140, 140, 141, 141, 142, 142, 143, 143, 144, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170];
        // assert!(Solution::is_possible(big_vec));
        // assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 5]));
        // assert!(Solution::is_possible(vec![1, 2, 3]));
        // assert!(Solution::is_possible(vec![1, 2, 3, 4]));
        // assert!(Solution::is_possible(vec![1, 2, 3, 6, 7, 8, 10, 11, 12]));
        // assert!(!Solution::is_possible(vec![1, 2, 3, 6, 7, 8, 10, 11]));
        // assert!(Solution::is_possible(vec![1, 2, 3, 4, 5, 6]));
        // assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]));
        // assert!(!Solution::is_possible(vec![1, 2, 3, 4, 4, 5]));
    }
}
