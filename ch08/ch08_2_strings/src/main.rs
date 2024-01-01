use std::fmt::format;

fn main() {
    creating_a_new_string();
    valid_utf8_strings();
    updating_strings();
    concatenation();
    internal_representation();
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

fn valid_utf8_strings() {
    // pasted in from The Book. println!() call added.
    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("שָׁלוֹם");
    println!("{hello}");
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");
    println!("{hello}");
}

fn updating_strings() {
    // append a str using push_str(). does not take ownership.
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("updated string: {s}");

    // push_str() takes a string slice so it does not take ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push() takes a char
    let mut s = String::from("lo");
    s.push('l');
    println!("updated with push(): {s}");
}

fn concatenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("concatenated string: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // s1 gets owned. s2 and s3 are sent as references.
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("concatenating multiple strings: {s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! takes references, so it does not take ownership of anything
    let s = format!("{s1}-{s2}-{s3}");
    println!("using format!: {s}");
}

fn internal_representation() {
    // you can't just index into a String
    //let s1 = String::from("hello");
    //let h = s1[0];

    // this word has 4 unicode bytes
    let hello = String::from("Hola");
    println!("String is: {hello}. Length is {0}", hello.len());

    // this word has 24
    let hello = String::from("Здравствуйте");
    println!("String is: {hello}. Length is {0}", hello.len());
    // this doesn't work
    //let answer = &hello[0];
}
