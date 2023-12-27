use rand::Rng;

// using nested paths when importing
#[allow(unused_imports)]
use std::{cmp::Ordering, array};

// using self to combine paths
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number: {}", secret_number);
}
