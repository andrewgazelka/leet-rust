use crate::Solution;

// 0 0 0 0   0
// 0 0 0 1   1        1
// 0 0 1 0   1        0
// 0 0 1 1   2        1

// 0 1 0 0   1        .. 1 = -1
// 0 1 0 1   2         1
// 0 1 1 0   2         0
// 0 1 1 1   3         1

// 1 0 0 0   1         ... 1 = -2
// 1 0 0 1   2         1
// 1 0 1 0   2         0
// 1 0 1 1   3         1

// 1 1 0 0   2         ... 2 = -1
// 1 1 0 1   3          1
// 1 1 1 0   3         0
// 1 1 1 1   4         1

// 1 0 0 0 0

//                     -3

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let num = num as usize;
        let mut vec = vec![0; num + 1];
        for i in 1..=num as usize {
            vec[i] = vec[i & (i - 1)] + 1;
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::count_bits(18));
    }
}
