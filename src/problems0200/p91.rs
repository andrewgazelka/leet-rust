use crate::utils::mask;
use crate::Solution;

/// 10101010101...
const MASK_P1: u8 = mask(1) as u8;

/// 110011001100...
const MASK_P2: u8 = mask(2) as u8;

/// 1111000011110000...
const MASK_P4: u8 = mask(4) as u8;

/// 111111110000000011111111...
const MASK_P8: u8 = mask(8) as u8;

fn print_step(num: usize){
    print!("  ");
    for i in 0..8 {
        if i % (num * 2) < num {
            print!("*")
        } else {
            print!(" ")
        }
    }
    println!();
}

macro_rules! print_lro {
    ($l: expr, $r: expr, $o: expr, $step: expr) => {
        print_step($step);
        println!("l {:08b}", $l);
        println!("+");
        println!("r {:08b}", $r);
        println!("=");
        println!("o {:08b}", $o);
    };
}

impl Solution {
    pub fn hamming_weight_kernighan(mut n: u32) -> u32 {
        let mut c = 0_u32;
        while n > 0 {
            n &= n - 1;
            c += 1;
        }
        c
    }


    /// Assuming the operators
    /// - `&`
    /// - `+`
    ///
    /// can execute in constant time,
    ///
    /// hamming weight can also be solved in `O(log n)` time, where `n` is the
    /// number of bits in a the given number, `c`.
    ///
    /// # Methodology
    /// The methodology for hamming weight is divide-and-conquer.
    /// - Take a number `1 0 1 1 0 1 1 0`
    ///
    /// Since this is a `O(log n)` problem, it is clear we want to solve this
    /// in a tree like manner. If we did this in parallel, we could do something
    /// like
    ///
    ///
    /// ```text
    /// INPUT 1 0 1 1 0 1 1 0
    ///
    ///  +(0b1, 0b0)      +(0b1, 0b1 )     +(0b0,0b1)       +(0b1,0b0)
    ///     \                  |            |               /
    ///     0b1               0b10         0b1             0b1
    ///       \                /            \              /
    ///            +(0b01, 0b10)                  +(0b01,0b01)
    ///                 \                           /
    ///                0b11                       0b10
    ///                     \                     /
    ///                         +(0b0011,0b0010)
    ///                                |
    ///                             0b101 = 5
    /// ```
    ///
    /// ```text
    /// how can we achieve something like
    ///        +     +     +     +
    /// 0b | 1 0 | 1 1 | 0 1 | 1 0
    ///
    /// suppose each * represents a 0
    ///
    /// 0b | * 1 | * 1 | * 0 | * 1      (A)
    /// 0b | * 0 | * 1 | * 1 | * 0      (A')
    ///
    /// then the resulting addition will yield
    ///
    /// 0b | 0 1 | 1 0 | 0 1 | 0 1      (A")
    ///
    /// we can achieve achieve (A) and (A') by applying a mask 0b01010101...
    /// to INPUT >> 1 and INPUT.
    ///
    /// we can repeat this with a mask 0b001100110011 on
    ///
    /// (A") and (A" >> 2)
    /// 0b | ** 0 1 | ** 0 1 |      (B)
    /// 0b | ** 1 0 | ** 0 1 |      (B')
    ///
    /// 0b | ** 1 1 | ** 1 0 |      (B")
    ///
    /// and continue
    ///
    /// 0b | **** 1 1 |  (C)
    /// 0b | **** 1 0 |  (C')
    /// 0b | ***1 0 1 |  (C")
    /// ```
    ///
    ///
    /// The example output of the function is
    /// ```text
    /// i 10110110
    /// * * * *
    /// l 00010100
    /// +
    /// r 01010001
    /// =
    /// o 01100101
    /// **  **
    /// l 00100001
    /// +
    /// r 00010001
    /// =
    /// o 00110010
    /// ****
    /// l 00000010
    /// +
    /// r 00000011
    /// =
    /// o 00000101
    /// ```
    pub fn hamming_weight_logn(c: u8) -> u8 {
        println!("i {c:08b}\n");

        let l =  c & MASK_P1;
        let r = (c >> 1) & MASK_P1;
        let o = l+r;

        print_lro!(l, r, o, 1);

        let l =  o & MASK_P2;
        let r = (o >> 2) & MASK_P2;
        let o = l+r;

        print_lro!(l, r, o, 2);

        let l =  o & MASK_P4;
        let r = (o >> 4) & MASK_P4;
        let o = l+r;

        print_lro!(l, r, o, 4);

        o
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let res = Solution::hamming_weight_logn(0b10110110);
        assert_eq!(res, 5);
    }
}
