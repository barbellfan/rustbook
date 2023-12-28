fn main() {
    creating_a_new_vector();
    initializing_a_new_vector();
}

fn creating_a_new_vector() {
    let v: Vec<i32> = Vec::new();
    println!("new Vec: {:?}", v);
}

fn initializing_a_new_vector() {
    let v = vec![1, 2, 3];
    println!("new Vec: {:?}", v);
}
