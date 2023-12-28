fn main() {
    creating_a_new_vector();
    initializing_a_new_vector();
    updating_a_vector();
}

fn creating_a_new_vector() {
    let v: Vec<i32> = Vec::new();
    println!("new Vec: {:?}", v);
}

fn initializing_a_new_vector() {
    let v = vec![1, 2, 3];
    println!("new Vec: {:?}", v);
}

fn updating_a_vector() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("updated vector: {:?}", v);
}
