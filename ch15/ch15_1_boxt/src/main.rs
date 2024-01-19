fn main() {
    using_box_t();
    recursive_types();
}

fn using_box_t() {
    let b = Box::new(5);
    println!("b = {}", b);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Why is this needed? I never needed it before when using a custom enum.
use crate::List::{Cons, Nil};

fn recursive_types() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);
}
