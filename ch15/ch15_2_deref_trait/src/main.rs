fn main() {
    follow_pointer_to_value();
}

fn follow_pointer_to_value() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
