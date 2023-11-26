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

}
