fn main() {
    matching_literals();
    matching_named_variables();
    multiple_patterns();
    matching_ranges();
    destructuring_struct_1();
    destructuring_struct_2();
    destructuring_enums();
    nested();
    destructuring_struts_and_tuples();
    ignoring_an_entire_value();
    ignoring_parts_of_a_value();
    ignoring_unused_vars();
    ignoring_remaining_parts_of_a_val_with_2_dots();
    extra_conditionals_with_match_guards();
    match_guard_2();
    match_guard_3();
    at_bindings_in_match();
}

fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {y}", x);
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_ranges() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else")
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_struct_1() {
    let p = Point { x: 0, y: 7 };

    // usual way
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // rust shortcut
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn destructuring_struct_2() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destructuring_enums() {
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match_em(msg);

    let msg = Message::Quit;
    match_em(msg);

    let msg = Message::Move { x: 10, y: 20 };
    match_em(msg);

    let msg = Message::Write(String::from("A message"));
    match_em(msg);
}

fn match_em(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message : {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
        _ => (),
    }
}

fn nested() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

}

fn destructuring_struts_and_tuples() {
    let ((feet, inches), Point{ x, y }) = ((3, 10), Point { x: 3, y: -10});

    println!("feet: {feet} inches: {inches} x: {x} y: {y}");
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
fn ignoring_an_entire_value() {
    foo(3, 4);
}

fn ignoring_parts_of_a_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match  (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("Setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
}

fn ignoring_unused_vars() {
    let _x = 5; // no compiler warning.
    //let y = 10; // compiler warning because it is unused.

    let s = Some(String::from("Hello!"));

    //if let Some(_s) = s { // compiler warning because s is moved into _s even tho _s is 'ignored'
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

fn ignoring_remaining_parts_of_a_val_with_2_dots()
{
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 1, z: 2 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
    match origin {
        Point { y, .. } => println!("y is {}", y),
    }
    match origin {
        Point { z, .. } => println!("z is {}", z),
    }

    let numbers = (2,4,8,16,32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
    /* this won't work, tho. 'second' is ambiguous
    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {second}");
        }
    }
    */
}

fn extra_conditionals_with_match_guards()
{
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

fn match_guard_2()
{
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn match_guard_3()
{
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn at_bindings_in_match()
{
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id)
    }
}
