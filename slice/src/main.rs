fn main() {
    let mut s = String::from("hello world");
    let word_index = first_world(&s);

    s.clear();
    println!("{}", word_index)
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
