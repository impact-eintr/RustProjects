fn main() {
    let s1 = String::from("Hello");
    let len = calcute_length(&s1);

    println!("The length of '{}' is {}", s1, len);
}

fn calcute_length(s: &String) -> usize {
    s.len()
}
