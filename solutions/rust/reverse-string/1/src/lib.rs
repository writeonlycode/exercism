use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for grapheme in input.graphemes(true) {
        result = format!("{}{}", grapheme, result);
    }

    result
}
