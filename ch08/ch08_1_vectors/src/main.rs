fn main() {
    creating_a_new_vector();
    initializing_a_new_vector();
    updating_a_vector();
    reading_elements_of_vectors();
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

fn reading_elements_of_vectors() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // crashes when run
    //let does_not_exist = &v[100];
    //println!("Does not exist: {does_not_exist}");

    let does_not_exist = v.get(100);
    println!("Does not exist: {:?}", does_not_exist);
}
