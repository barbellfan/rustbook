use std::{thread, time::Duration};

fn main() {
    exp_closure();
    various_closures();
}

fn exp_closure() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let ex_cl_res = expensive_closure(5);

    println!("Result from expensive closure: {}", ex_cl_res);
}

fn various_closures() {
    // comparing a function to various versions of a closure.
    // they all do the same thing
    fn  add_one_v1   (x: u32) -> u32 { x + 1}
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };// only compiles when used. type depends on what you send it
    let add_one_v4 = |x|               x + 1  ;// only compiles when used. type depends on what you send it

    let add_v1 = add_one_v1(4);
    println!("add_one_v1: {}", add_v1);

    let add_v2 = add_one_v2(4);
    println!("add_one_v2: {}", add_v2);

    let add_v3a = add_one_v3(4); // this is coerced to a u32 because of the next usage of the closure.
    println!("add_one_v3. param is coerced to u32: {}", add_v3a);

    let add_me: u32 = 4;
    let add_v3b = add_one_v3(add_me); // this coerces add_one_v3 to need a u32 param
    println!("add_one_v3. param defined as a u32: {}", add_v3b);

    let add_v4 = add_one_v4(4); // just using default data type, i32.
    println!("add_one_v4: {}", add_v4);

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    //let n = example_closure(5); // this won't compile because the previous line set the data type of the closure.
    println!("example closure result: {s}");
}
