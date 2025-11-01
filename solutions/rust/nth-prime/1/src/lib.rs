pub fn nth(n: u32) -> u32 {
    let mut i = 0;
    let mut counter = 0;

    loop {
        if is_prime(i) {
            counter += 1;
        }

        if counter == n + 1 {
            break i;
        }

        i += 1;
    }
}

pub fn is_prime(n: u32) -> bool {
    n == 2 || n > 2 && !(2..n.isqrt() + 1).any(|e| n % e == 0)
}
