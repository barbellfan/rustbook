fn main() {
    let reference_to_nothging = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
