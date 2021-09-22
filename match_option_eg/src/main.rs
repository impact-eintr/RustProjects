fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    match_all();
}

fn match_all() {
    let v = 0u8;
    match v {
        1 => println!("1"),
        3 => println!("3"),
        5 => println!("5"),
        7 => println!("7"),
        _ => (),
    }
}
