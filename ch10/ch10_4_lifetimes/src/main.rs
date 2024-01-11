fn main() {
    prevent_dangling_references();
    generic_lifetimes_in_functions();
    different_lifetimes();
    different_lifetimes_2();
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

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn different_lifetimes() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn different_lifetimes_2() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result); // works here
    }
    //println!("The longest string is {}", result); // but not here, since string2 is out of scope
}
