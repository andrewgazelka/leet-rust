use crate::Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;

        let mut str = String::with_capacity(n);

        let factorials = {
            let mut temp = Vec::with_capacity(n);
            temp.push(1);
            for i in 1..=n {
                temp.push((i) * temp[i - 1]);
            }
            temp
        };

        let mut k_left = k as usize - 1; // 0-indexed

        let mut available_numbers: Vec<_> = (1..=n as u8).collect();

        for factorial_idx in (0..n).rev() {
            let factorial = factorials[factorial_idx];

            let num_to_push_idx = k_left / factorial;
            let num_to_push = available_numbers.remove(num_to_push_idx);

            let char_to_push = (num_to_push + b'0') as char;
            str.push(char_to_push);
            k_left %= factorial;
        }

        str
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::get_permutation(3, 3), "213");
        assert_eq!(Solution::get_permutation(4, 9), "2314");
        assert_eq!(Solution::get_permutation(3, 1), "123");
    }
}
