const NUM: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

const CNUM: [&str; 11] = [
    "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();

    for i in 0..take_down {
        song.push_str(
            format!(
                "{0} green bottle{1} hanging on the wall,\n{0} green bottle{1} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {2} green bottle{3} hanging on the wall.\n\n",
                CNUM[(start_bottles - i) as usize],
                if start_bottles - i != 1 { "s" } else { "" },
                NUM[(start_bottles - 1 - i) as usize],
                if start_bottles - 1 - i != 1 { "s" } else { "" }
            )
            .as_str(),
        );
    }

    song
}
