use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    if let Some(s) = score {
        println!("{}", s);
    } else {
        println!("team not exist");
    }
}
