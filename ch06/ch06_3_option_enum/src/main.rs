fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_char: {:?}", some_char);
    println!("absent_number: {:?}", absent_number);

    /* This won't compile since y is not the same type as x.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
    */
}
