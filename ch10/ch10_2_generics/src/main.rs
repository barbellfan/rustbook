fn main() {
    generic_in_fn_def();
    generic_in_struct_def();
    generic_in_struct_def_2();
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_in_fn_def() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

struct Point<T> {
    x: T,
    y: T,
}

fn generic_in_struct_def() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.01, y: 4.01 };
    //let wont_work = Point { x: 5, y: 4.0 }; // types must match

    println!("integer.x: {}", integer.x);
    println!("integer.y : {}", integer.y);
    println!("float.x: {}", float.x);
    println!("float.y: {}", float.y);
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn generic_in_struct_def_2() {
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.01, y: 4.01 };
    let integer_and_float = Point2 { x: 5, y: 4.01 };

    println!("integer.x: {}", both_integer.x);
    println!("integer.y : {}", both_integer.y);
    println!("both_float.x: {}", both_float.x);
    println!("both_float.y: {}", both_float.y);
    println!("integer_and_float.x: {}", integer_and_float.x);
    println!("integer_and_float.y: {}", integer_and_float.y);
}
