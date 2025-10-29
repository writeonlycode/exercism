const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn annotate<'a>(garden: &[&'a str]) -> Vec<String> {
    let mut result: Vec<Vec<char>> = garden
        .iter()
        .map(|line| line.chars().map(|slot| slot).collect())
        .collect();

    for (i, row) in garden.iter().enumerate() {
        for (j, slot) in row.chars().enumerate() {
            match slot {
                '*' => {
                    for (dx, dy) in DIRECTIONS {
                        let ii: isize = i as isize + dx;
                        let jj: isize = j as isize + dy;

                        if ii >= 0 && jj >= 0 {
                            if let Some(value) = result.get_mut(ii as usize) {
                                if let Some(value) = value.get_mut(jj as usize) {
                                    *value = increment(*value);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    result.iter().map(|line| line.iter().collect()).collect()
}

fn increment(c: char) -> char {
    if c == ' ' {
        '1'
    } else if c.is_ascii_digit() {
        char::from_digit(c.to_digit(10).unwrap() + 1, 10).unwrap()
    } else {
        c
    }
}
