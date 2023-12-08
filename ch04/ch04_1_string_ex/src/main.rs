fn main() {
    let mut s = String::from("hello"); // s is valid starting here
    s.push_str(", world!");
    println!("{}", s);
} // s is now out of scope. rust calls drop().
