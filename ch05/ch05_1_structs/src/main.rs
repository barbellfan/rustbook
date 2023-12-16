fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    user1.email = String::from("anotheremail@example.com");

    println!("username: {}", user1.username);
    println!("active: {}", user1.active);
    println!("email: {}", user1.email);
    println!("sign in count: {}", user1.sign_in_count);

    let user2 = build_user(String::from("user2@example.com"), String::from("user2name"));
    println!("user2: {}", user2.username);

    let user3 = build_user2(String::from("user3@example.com"), String::from("user3name"));
    println!("user3: {}", user3.username);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
