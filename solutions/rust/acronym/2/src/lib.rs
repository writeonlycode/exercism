pub fn abbreviate(phrase: &str) -> String {
    let p = phrase.split([' ', '-', '_']);
    println!("{:?}", p);
    // let mut s = String::new();
    // let mut chars = phrase.chars();
    //
    // if let Some(c) = chars.next() {
    //     s.push_str(c.to_ascii_uppercase().to_string().as_str());
    // }
    //
    // while let Some(c) = chars.next() {
    //     if c.is_whitespace() {
    //         if let Some(c) = chars.next() {
    //             if c.is_alphabetic() {
    //                 s.push_str(c.to_ascii_uppercase().to_string().as_str());
    //             }
    //         }
    //     } else if c == '-' {
    //         if let Some(c) = chars.next() {
    //             if c.is_alphabetic() {
    //                 s.push_str(c.to_ascii_uppercase().to_string().as_str());
    //             }
    //         }
    //     } else if c.is_uppercase() {
    //         if let Some(n) = chars.next() {
    //             if !n.is_uppercase() {
    //                 s.push_str(c.to_ascii_uppercase().to_string().as_str());
    //             }
    //         }
    //     }
    // }
    //
    // s
    String::new()
}
