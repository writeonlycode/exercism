pub fn collatz(n: u64) -> Option<u64> {
    if n > 0 { Some(collatz_r(0, n)) } else { None }
}

fn collatz_r(acc: u64, n: u64) -> u64 {
    if n == 1 {
        acc
    } else if n % 2 == 0 {
        collatz_r(acc + 1, n / 2)
    } else {
        collatz_r(acc + 1, (n * 3) + 1)
    }
}
