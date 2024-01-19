fn main() {
    using_box_t();
}

fn using_box_t() {
    let b = Box::new(5);
    println!("b = {}", b);
}
