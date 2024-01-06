use std::collections::{HashMap, btree_map::{Entry, OccupiedEntry, VacantEntry}};

fn main() {
    creating_new_hashmap();
    getting_values_from_hashmaps();
    hashmaps_and_ownership();
    overwriting_a_value();
    add_key_if_not_present();
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

fn hashmaps_and_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("map: {map:?}");

    // these fail at compilation with "borrow of moved value"
    //println!("field_name: {field_name}");
    //println!("field_value: {field_value}");

    // Hashmaps own their variables. Unless you insert a reference.
    // Then the reference must be valid as long as the Hashmap is valid.
}

fn overwriting_a_value() {
    // overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Blue"),25);

    println!("scores after overwrite: {scores:?}");
}

fn add_key_if_not_present() {
    use std::collections::hash_map::Entry::*;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("key added if not present: {scores:?}");

    // not in the docs, just wanted to try it
    // try replacing or_insert() with explicit match statements.
    match scores.entry(String::from("Yellow")) {
        Occupied(entry) => println!("occupied: {entry:?}"),
        Vacant(entry) => println!("vacant: {entry:?}"),
    }

    match scores.entry(String::from("Green")) {
        Occupied(entry) => println!("occupied: {entry:?}"),
        Vacant(entry) => println!("vacant: {entry:?}"),
    }
    // Shows that you can take any arbitrary action depending whether the key exists.
}
