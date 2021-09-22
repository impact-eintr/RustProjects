fn main() {
    let v = vec![1, 2, 3, 4, 5];
    match v.get(100) {
        Some(third) => println!("{}", third),
        None => println!("There is no third element"),
    }
}
