fn main() {
    function_pointers();
    returning_closures();
}

fn function_pointers() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

fn returning_closures() {
    /* does not compile since you can't return a bare closure
    fn returns_closure() -> dyn Fn(i32) -> i32 {
        |x| x + 1
    }
    */

    // this work because you are returning a Box object
    fn _returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
