fn main() {
    let x = 2.0; // default is f64.
    println!("x: {x}");

    let y: f32 = 3.0; // but you can make it an f32.
    println!("y: {y}");
    // You could probably print them better so they look like floating points.

    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    // booleans!
    let t = true;
    let f: bool = false;
    println!("t: {t}, f: {f}");

    // chars
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}. z: {z}, heart eyed cat: {heart_eyed_cat}");

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // dereferencing
    println!("The values in the tuple - x: {x}, y: {y}, z: {z}");
    // getting the values directly
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("dereferenced values: {five_hundred}, {six_point_four}, {one}");

    // arrays
    let a1 = [1, 2, 3, 4, 5]; // simple array initialization
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"]; // initializing with strings
    let a2: [i32; 5] = [1, 2, 3, 4, 5]; // initialize with manual data type
    let a3 = [3; 5]; // fill with three fives
    println!("a1: {:?}", a1); // fancy way to print arrays
    println!("months: {:?}", months);
    println!("a2: {:?}", a2);
    println!("a3: {:?}", a3);

    let first = a1[0];
    let second = a1[1];
    println!("first element: {first}, second element: {second}");
}
