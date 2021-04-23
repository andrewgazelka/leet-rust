use crate::Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut rows = Vec::with_capacity(num_rows as usize);

        for _ in 0..num_rows {
            rows.push(vec!());
        }

        let mut row_on = 0;
        let mut zigzag = false;
        for x in s.chars() {
            rows[row_on as usize].push(x);
            row_on += if zigzag { -1 } else { 1 };
            if row_on == 0 || row_on == (num_rows - 1) {
                zigzag = !zigzag;
            }
        }
        rows.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod tests {

    use crate::Solution;

    #[test]
    fn paypal() {
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
        assert_eq!(Solution::convert("AB".to_string(), 1), "AB");
    }
}
