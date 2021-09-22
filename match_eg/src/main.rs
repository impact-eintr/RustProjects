#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}



fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("test");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 20,
        Coin::Quarter(state) => {
            println!("A quarter from {:?}", state);
            25
        },
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("Hello, {}!", value_in_cents(c));
}
