#![allow(unused_mut)]

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Generates warning, not compiler error like in previous
    // version of Rust. Current Rust docs say this should fail
    // compilation.
    // I disabled the warning with the attribute.
    let mut spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}
