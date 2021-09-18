fn main() {
    let s1 = String::from("Hello");
    take_owership(s1);
    println!("{}", s1);
    let x = 6;

    makes_copy(x);
    println!("{}", x)

}

fn take_owership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}
