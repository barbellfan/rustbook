fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };
    println!("username: {}", user1.username);
    println!("active: {}", user1.active);
    println!("email: {}", user1.email);
    println!("sign in count: {}", user1.sign_in_count);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
