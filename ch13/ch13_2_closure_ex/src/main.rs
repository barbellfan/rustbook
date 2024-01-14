use std::{thread, time::Duration};

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let ex_cl_res = expensive_closure(5);

    println!("Result from expensive closure: {}", ex_cl_res);
}

