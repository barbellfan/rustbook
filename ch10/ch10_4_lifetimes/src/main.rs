fn main() {
    prevent_dangling_references();
}

fn prevent_dangling_references() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
