#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;

    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m < n => if b.windows(a.len()).any(|x| x == a ) { Sublist } else { Unequal },
        (m, n) if m > n => if a.windows(b.len()).any(|x| x == b ) { Superlist } else { Unequal },
        (_, _) => if a == b { Equal } else { Unequal },
    }
}
