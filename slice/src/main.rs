fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    for (_i, item) in slice.iter().enumerate() {
    println!("{}",item);
    }
}
