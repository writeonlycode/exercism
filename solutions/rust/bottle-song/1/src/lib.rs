pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();

    for i in 0..take_down {
        song.push_str(
            format!(
                "{0} green bottle{1} hanging on the wall,
{0} green bottle{1} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {2} green bottle{3} hanging on the wall.\n\n",
                number_to_string(start_bottles - i, true),
                if start_bottles - i != 1 { "s" } else { "" },
                number_to_string(start_bottles - 1 - i, false),
                if start_bottles - 1 - i != 1 { "s" } else { "" }
            )
            .as_str(),
        );
    }

    song
}

fn number_to_string(n: u32, capitalize: bool) -> &'static str {
    match n {
        0 => {
            if capitalize {
                "No"
            } else {
                "no"
            }
        }
        1 => {
            if capitalize {
                "One"
            } else {
                "one"
            }
        }
        2 => {
            if capitalize {
                "Two"
            } else {
                "two"
            }
        }
        3 => {
            if capitalize {
                "Three"
            } else {
                "three"
            }
        }
        4 => {
            if capitalize {
                "Four"
            } else {
                "four"
            }
        }
        5 => {
            if capitalize {
                "Five"
            } else {
                "five"
            }
        }
        6 => {
            if capitalize {
                "Six"
            } else {
                "six"
            }
        }
        7 => {
            if capitalize {
                "Seven"
            } else {
                "seven"
            }
        }
        8 => {
            if capitalize {
                "Eight"
            } else {
                "eight"
            }
        }
        9 => {
            if capitalize {
                "Nine"
            } else {
                "nine"
            }
        }
        10 => {
            if capitalize {
                "Ten"
            } else {
                "ten"
            }
        }
        _ => {
            if capitalize {
                "N"
            } else {
                "n"
            }
        }
    }
}
