fn main() {
    creating_a_new_string();
}

fn creating_a_new_string() {
    // simple way to create an empty string
    let mut _s = String::new();

    // &str implements Display, so you can call to_string() on it
    let data = "initial contents";
    let s = data.to_string();
    println!("calling to_string on a literal: {s}");

    // you can also call to_string() on the literal directly
    let s = "initial contents".to_string();
    println!("calling to_string() directly on the literal: {s}");

    // you can ALSO use String::from to create a String from a string literal
    let s = String::from("initial contents");
    println!("using String::from() on a literal: {s}");
}
