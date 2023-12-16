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

    // create new struct using some values from other struct
    let user4 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("user4 email: {}", user4.email);

    // struc update syntax
    let user5 = User {
        email: String::from("yetanother@example.com"),
        ..user2
    };

    println!("user5 email: {}", user5.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// return a struct from a function
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// uses field init shorthand
fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
