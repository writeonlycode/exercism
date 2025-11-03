pub fn reply(message: &str) -> &str {
    let m = message.trim();

    if m.chars().all(|c| c.is_whitespace()) {
        return "Fine. Be that way!";
    }

    let is_question = m.ends_with("?");

    if m.chars().any(|c| c.is_alphabetic())
        && m.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
    {
        if is_question {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if is_question {
        "Sure."
    } else {
        "Whatever."
    }
}
