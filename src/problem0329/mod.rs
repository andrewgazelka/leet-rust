use std::cmp::max;

use crate::Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();

        let l = n * m;

        if l == 1 {
            return 1;
        }


        let largest_keys: Vec<_> = {
            let mut temp: Vec<_> = matrix.iter().cloned().flatten().enumerate().collect();
            temp.sort_by_key(|&(_, v)| -v);
            temp
        };

        let mut dp = vec![1; l];

        largest_keys.into_iter().for_each(|(key, val)| {
            let i = key / m; // n coord
            let j = key % m; // m coord

            let mut max_dp = 0;
            if i + 1 != n && matrix[i + 1][j] > val {
                max_dp = max(max_dp, dp[key + m]);
            }

            if j + 1 != m && matrix[i][j + 1] > val {
                max_dp = max(max_dp, dp[key + 1]);
            }

            if i > 0 && matrix[i - 1][j] > val {
                max_dp = max(max_dp, dp[key - m]);
            }

            if j > 0 && matrix[i][j - 1] > val {
                max_dp = max(max_dp, dp[key - 1]);
            }

            dp[key] += max_dp;
        });


        dp.into_iter().max().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let mat1 = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
        let mat2 = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
        let mat3 = vec![vec![1]];
        let mat4 = vec![vec![1, 2]];
        let mat5 = vec![vec![1], vec![2]];

        assert_eq!(Solution::longest_increasing_path(mat1), 4);
        assert_eq!(Solution::longest_increasing_path(mat2), 4);
        assert_eq!(Solution::longest_increasing_path(mat3), 1);
        assert_eq!(Solution::longest_increasing_path(mat4), 2);
        assert_eq!(Solution::longest_increasing_path(mat5), 2);
    }
}
