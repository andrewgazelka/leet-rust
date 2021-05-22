use crate::Solution;
use std::cmp::max;

#[inline]
fn idx(character: char) -> usize {
    character as usize - 'a' as usize
}

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s: Vec<_> = s.chars().collect();
        let len = s.len();
        let k = k as usize;

        if len < k {
            return 0
        }

        let mut char_map = [0_usize; 26];

        let mut total_unique = 0;
        s.iter().for_each(|&x| {
            let idx = idx(x);
            total_unique += char_map[idx] ^ 1;
            char_map[idx] = 1
        });

        let mut max_len = 0_usize;

        for unique_required in 1..=total_unique {
            char_map.iter_mut().for_each(|x|*x = 0);
            let mut unique_count = 0;
            let mut at_least_k_count = 0;
            let mut window_start = 0;
            let mut window_end = 0;

            while  window_end < len {
                if unique_count <= unique_required {

                    let c = s[window_end];
                    let c_count = &mut char_map[idx(c)];

                    if *c_count == 0 {
                       unique_count += 1;
                    }

                    *c_count += 1;

                    if *c_count == k {
                        at_least_k_count += 1;
                    }

                    window_end +=1;

                } else {

                    let c = s[window_start];
                    let c_count = &mut char_map[idx(c)];

                    if *c_count == k {
                        at_least_k_count -=1;
                    }

                    *c_count -=1;

                    if *c_count == 0 {
                        unique_count -= 1;
                    }

                    window_start+=1;
                }

                if unique_count == at_least_k_count {
                    max_len = max(max_len, window_end - window_start);
                }
            }
        }
        max_len as i32
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let solve = |s: &str, k: i32| Solution::longest_substring(s.to_string(), k);

        assert_eq!(solve("aaabb", 3), 3);
        assert_eq!(solve("ababbc", 2), 5);
        assert_eq!(solve("ab", 1), 2);
        assert_eq!(solve("ab", 2), 0);
        assert_eq!(solve("", 2), 0);
    }
}
