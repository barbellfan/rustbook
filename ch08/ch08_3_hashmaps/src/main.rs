use std::collections::HashMap;

fn main() {
    creating_new_hashmap();
    getting_values_from_hashmaps();
}

fn creating_new_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {scores:?}");
}

fn getting_values_from_hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("using get()");
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score for {team_name}: {score}");

    println!("iterating over all values");
    for (key, value) in &scores {
        println!("{key}: {value}")
    }
}
