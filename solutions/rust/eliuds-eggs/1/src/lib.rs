pub fn egg_count(display_value: u32) -> usize {
    format!("{:b}", display_value)
        .chars()
        .fold(0, |a, e| if e == '1' { a + 1 } else { a })
}
