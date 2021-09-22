fn main() {
    let mut v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => println!("others"),
    }

    v = Some(3);

    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}
