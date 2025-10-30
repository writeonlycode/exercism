pub fn square(s: u32) -> u64 {
    (1..=s as u64).reduce(|acc, _| acc * 2).unwrap()
}

pub fn total() -> u64 {
    (1..=64).map(|e| square(e)).sum()
}
