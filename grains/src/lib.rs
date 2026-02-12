pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 1_u64 << (s-1),
        _ => panic!("out of u64 range")
    }
}

pub fn total() -> u64 {
    // let last = square(64);
    // last + (last - 1)
    u64::MAX 
}
