pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split([' ', '-', '_'])
        .flat_map(|w| {
            w.chars().take(1).chain(
                w.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
