struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = new_user(&String::from("23333333@gmail.com")[..], &String::from("Eintr")[..]);
    let user2 = User {
        username:String::from("eintr"),
        ..user1
    };
    println!("Hello, {} and {}", user1.username, user2.username);
}

fn new_user(email: &str, username: &str) -> User {
    User {
        email:email.to_string(),
        username:username.to_string(),
        active:true,
        sign_in_count:1,
    }
}
