pub fn annotate<'a>(garden: &[&'a str]) -> Vec<String> {
    let mut result = Vec::new();

    for line in garden.iter() {
        let mut new_line = Vec::new();

        for column in line.chars() {
            new_line.push(column.clone());
        }

        result.push(new_line);
    }

    for (i, row) in garden.iter().enumerate() {
        for (j, slot) in row.chars().enumerate() {
            match slot {
                '*' => {
                    if i > 0 && j > 0 {
                        result[i - 1][j - 1] = increment(result[i - 1][j - 1])
                    }
                    if i > 0 {
                        result[i - 1][j] = increment(result[i - 1][j])
                    }
                    if i > 0 && j + 1 < row.len() {
                        result[i - 1][j + 1] = increment(result[i - 1][j + 1])
                    }
                    if j > 0 {
                        result[i][j - 1] = increment(result[i][j - 1])
                    }
                    if j + 1 < row.len() {
                        result[i][j + 1] = increment(result[i][j + 1])
                    }
                    if i + 1 < garden.len() && j > 0 {
                        result[i + 1][j - 1] = increment(result[i + 1][j - 1])
                    }
                    if i + 1 < garden.len() {
                        result[i + 1][j] = increment(result[i + 1][j])
                    }
                    if i + 1 < garden.len() && j + 1 < row.len() {
                        result[i + 1][j + 1] = increment(result[i + 1][j + 1])
                    }
                }
                _ => {}
            }
        }
    }

    let mut final_result = Vec::new();

    for line in result {
        let line: String = line.iter().collect();
        final_result.push(line);
    }

    final_result
}

fn increment(input: char) -> char {
    match input {
        ' ' => '1',
        '0' => '1',
        '1' => '2',
        '2' => '3',
        '3' => '4',
        '4' => '5',
        '5' => '6',
        '6' => '7',
        '7' => '8',
        value => value,
    }
}
