pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .fold(Vec::new(), |mut acc, e| match e {
            '{' => {
                acc.push('{');
                acc
            }
            '}' => match acc.last() {
                Some('{') => {
                    acc.pop();
                    acc
                }
                _ => {
                    acc.push('}');
                    acc
                }
            },
            '[' => {
                acc.push('[');
                acc
            }
            ']' => match acc.last() {
                Some('[') => {
                    acc.pop();
                    acc
                }
                _ => {
                    acc.push(']');
                    acc
                }
            },
            '(' => {
                acc.push('(');
                acc
            }
            ')' => match acc.last() {
                Some('(') => {
                    acc.pop();
                    acc
                }
                _ => {
                    acc.push(')');
                    acc
                }
            },
            _ => acc,
        })
        .is_empty()
}
