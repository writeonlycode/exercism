pub fn reply(message: &str) -> &str {
    let m = message.trim();

    if m.chars().all(|c| c.is_whitespace()) {
        "Fine. Be that way!"
    } else if m.chars().any(|c| c.is_alphabetic())
        && m.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
    {
        if m.chars().last().unwrap() == '?' {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else {
        if m.chars().last().unwrap() == '?' {
            "Sure."
        } else {
            "Whatever."
        }
    }
}
