pub fn is_armstrong_number(num: u32) -> bool {
    let n = num.to_string();

    let sum = n.chars().fold(0, |acc, e| {
        acc + e.to_string().parse::<u32>().unwrap().pow(n.len() as u32)
    });

    sum == num
}
