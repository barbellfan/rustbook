// When bringing in structs, enums, and other items with
// use, it's idiomatic to specify the full path, like with
// HashMap below.

// Except when bringing two items with the same name, like
// with std:fmt and std::io.
use std::collections::HashMap;

use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("map: {:?}", map);
}

#[allow(unused)]
fn function1() -> Result {
    Ok(())
}

#[allow(unused)]
fn function2() -> IoResult<()> {
    Ok(())
}
