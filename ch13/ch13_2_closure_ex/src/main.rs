use std::{thread, time::Duration, vec};

fn main() {
    exp_closure();
    various_closures();
    sharing_immutable_refs();
    sharing_mutable_refs();
    force_closure_ownership();
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

fn sharing_immutable_refs() {
    println!("Immutable references can be shared.");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn sharing_mutable_refs() {
    println!("Mutable references can be shared too.");

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // cannot borrow `list` as immutable because it is also borrowed as mutable
    //println!("After defining closure: {:?}", list);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn force_closure_ownership() {
    println!("Moving ownership");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // move keyword is required here, since list is going to another thread.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // cannot use list here:
    // borrow of moved value
    //println!("Can I use list here?: {:?}", list);
}
