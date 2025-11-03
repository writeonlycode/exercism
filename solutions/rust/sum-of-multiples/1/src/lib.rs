use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .map(|e| get_multiples_less_than(*e, limit))
        .flatten()
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}

fn get_multiples_less_than(base: u32, limit: u32) -> Vec<u32> {
    (1..limit)
        .into_iter()
        .map(|e| base * e)
        .filter(|&e| e < limit)
        .collect::<Vec<u32>>()
}
