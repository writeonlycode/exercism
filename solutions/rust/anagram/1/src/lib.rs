use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    for possible_anagram in possible_anagrams.iter() {
        if is_anagram(word, possible_anagram) {
            result.insert(possible_anagram);
        }
    }

    result
}

fn is_anagram(word: &str, target: &str) -> bool {
    let word = word.to_lowercase();
    let target = target.to_lowercase();

    if word == target {
        return false;
    }

    let mut word_chars: Vec<char> = word.clone().chars().collect();
    let target_chars = target.chars();

    for c in target_chars {
        let position = word_chars.iter().position(|&t| c == t);

        if let Some(position) = position {
            word_chars.remove(position);
        } else {
            return false;
        }
    }

    word_chars.is_empty()
}
