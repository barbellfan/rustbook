fn main() {
    prevent_dangling_references();
    generic_lifetimes_in_functions();
}

fn prevent_dangling_references() {
    let x = 5;

    let r = &x;

    println!("r: {}", r);
}

fn generic_lifetimes_in_functions() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
