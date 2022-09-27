#![deny(unsafe_code)]
#![no_std]

pub fn scale(value: u32, min_val: u32, max_val: u32, new_min_val: u32, new_max_val: u32) -> u32 {
    return (value - min_val) * (new_max_val - new_min_val) / (max_val - min_val) + new_min_val;
}
