fn main() {
    let reference_to_nothing = dangle();

    println!("the string: {}", reference_to_nothing);
}

fn dangle() -> String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    s // return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
