fn main() {
    prevent_dangling_references();
}

fn prevent_dangling_references() {
    let x = 5;

    let r = &x;

    println!("r: {}", r);
}
