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

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
