pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .fold(Vec::new(), |mut a, e| {
            a.push(e.iter().map(|c| c.to_string()).collect());
            a
        })
}
