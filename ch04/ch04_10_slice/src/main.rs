fn main() {
    let phrase = String::from("Hello, world!");
    let index = first_word(&phrase);
    println!("Index of end of first word: {}", index);
}

/// Take a string of words separated by spaces
/// Return the index of the end of first word in the string.
/// If no spaces, return the length of the string.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
