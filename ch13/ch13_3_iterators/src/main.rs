fn main() {
    iterator_ex();
}

fn iterator_ex() {
    println!("iterator example");

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // lazy iterator does nothing until you use it.

    for val in v1_iter {
        println!("Got: {}", val);
    }
}
