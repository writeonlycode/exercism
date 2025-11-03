pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = Vec::new();

    while n > 1 {
        let divisor = (2..=n).find(|e| n % e == 0).unwrap();
        factors.push(divisor);
        n = n / divisor;
    }

    factors
}
