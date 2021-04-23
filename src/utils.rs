const NUM_BITS: u32 = 32;

const fn create_mask_helper(mut input: u32, set: bool, width: u8, iter: u32, sub_iter: u8) -> u32 {
    input <<= 1;
    if set {
        input |= 0b1;
    }
    if iter == 0 {
        return input;
    }

    if sub_iter == 0 {
        create_mask_helper(input, !set, width, iter - 1, width)
    } else {
        create_mask_helper(input, set, width, iter - 1, sub_iter - 1)
    }
}

/// masks start with 0
pub const fn create_mask(period: u8) -> u32 {
    let iter_0idx = NUM_BITS - 1;
    let width_0idx = period - 1;
    create_mask_helper(0, false, width_0idx, iter_0idx, width_0idx)
}
