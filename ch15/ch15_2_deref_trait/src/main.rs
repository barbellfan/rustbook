fn main() {
    follow_pointer_to_value();
    use_boxt_like_a_reference();
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
