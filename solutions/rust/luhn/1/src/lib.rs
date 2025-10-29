/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: Vec<char> = code.chars().filter(|d| !d.is_whitespace()).collect();

    if code.len() <= 1 {
        return false;
    }

    if code.iter().any(|d| !d.is_ascii_digit()) {
        return false;
    }

    let (_, total) = code
        .iter()
        .rev()
        .map(|&d| String::from(d).parse::<usize>().unwrap())
        .fold((false, 0), |(should_double, sum), e| {
            if should_double {
                if e * 2 > 9 {
                    (!should_double, sum + (e * 2) - 9)
                } else {
                    (!should_double, sum + (e * 2))
                }
            } else {
                (!should_double, sum + e)
            }
        });

    total % 10 == 0
}
