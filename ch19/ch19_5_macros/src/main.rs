fn main() {
    declarative_macro();
    derive_macro();
}

fn declarative_macro() {
    let v: Vec<u32> = macros::vec![1, 2, 3];

    println!("Vec from macro: {:?}", v)
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

fn derive_macro() {
    #[derive(HelloMacro)]
    struct Pancakes;

    Pancakes::hello_macro(); // Returns: 'hello, macro Pancakes' which is wrong
}
