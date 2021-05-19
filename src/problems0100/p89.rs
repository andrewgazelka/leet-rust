use crate::Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let iter = 1_i32 << n;
        let mut vec = Vec::with_capacity(iter as usize);

        for i in 0..iter {
            // kind of like munching squares
            vec.push(i ^ (i >> 1));
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        for x in Solution::gray_code(3) {
            println!("{:08b}", x);
        }
    }
}
