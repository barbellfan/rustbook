fn main() {
    let st = String::from("hello world");
    let fw = first_word(&st);
    //st.clear(); // causes compiler error with borrowing
    println!("{}", fw);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
