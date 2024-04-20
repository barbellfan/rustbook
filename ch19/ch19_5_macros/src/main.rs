fn main() {
    declarative_macro();
}

fn declarative_macro() {
    let v: Vec<u32> = macros::vec![1, 2, 3];

    println!("Vec from macro: {:?}", v)
}
