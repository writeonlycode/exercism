pub fn build_proverb(list: &[&str]) -> String {
    let mut s = String::new();

    if list.len() == 0 {
        return s;
    }

    for w in list.windows(2) {
        s.push_str(format!("For want of a {} the {} was lost.\n", w[0], w[1]).as_str());
    }

    s.push_str(format!("And all for the want of a {}.", list.first().unwrap()).as_str());

    s
}
