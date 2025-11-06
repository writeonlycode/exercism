const STUDENTS: [&'static str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let i = STUDENTS.iter().position(|&s| s == student);

    if let Some((first_line, second_line)) = diagram.split_once('\n') {
        let r = first_line
            .chars()
            .skip(i.unwrap() * 2)
            .take(2)
            .chain(second_line.chars().skip(i.unwrap() * 2).take(2));

        r.map(|c| match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => panic!(""),
        })
        .collect()
    } else {
        Vec::new()
    }
}
