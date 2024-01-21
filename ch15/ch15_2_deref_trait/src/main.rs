use std::ops::Deref;

fn main() {
    follow_pointer_to_value();
    use_boxt_like_a_reference();
    define_our_own_smart_pointer();
    implicit_deref_coercions();
}

fn follow_pointer_to_value() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn use_boxt_like_a_reference() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn define_our_own_smart_pointer() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // last line does this for *y: *(y.deref())
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn implicit_deref_coercions() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Deref on String returns a string slice

    // alternate way without using deref coercion
    hello(&(*m)[..]); // which looks ridiculous
}
