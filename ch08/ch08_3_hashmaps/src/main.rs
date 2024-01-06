use std::collections::HashMap;

fn main() {
    creating_new_hashmap();
}

fn creating_new_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {scores:?}");
}
