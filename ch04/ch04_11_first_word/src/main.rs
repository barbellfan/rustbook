fn main() {
    let st = String::from("hello world");
    let fw = first_word(&st);
    println!("{}", fw);

    // first_word works on slices of strings, whether partial or whole
    let word = first_word(&st[0..6]);
    println!("partial slice: {}", word);

    let word = first_word(&st[..]);
    println!("whole slice: {}", word);

    // first_word also works on refrences to string, which are
    // equivalent to a whole slice of the stringg
    let word = first_word(&st);
    println!("reference to the String: {}", word);

    let string_literal = "hello world";

    // first_word works on slices of string literals, whether partial or whole
    let word = first_word(&string_literal[0..6]);
    println!("slice of string literal: {}", word);
    let word = first_word(&string_literal[..]);
    println!("whole string literal: {}", word);

    // because string literals are string slices already,
    // this works too, without the slice syntax
    let word = first_word(string_literal);
    println!("string literal is a slice: {}", word);

    // slices work with arrays, too
    let a = [1, 2, 3, 4, 5];
    let sl = &a[1..3];
    assert_eq!(sl, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
