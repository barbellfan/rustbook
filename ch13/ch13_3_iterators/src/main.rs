fn main() {
    iterator_ex();
    methods_that_produce_other_iterators();
}

fn iterator_ex() {
    println!("iterator example");

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // lazy iterator does nothing until you use it.

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn methods_that_produce_other_iterators() {
    let v1: Vec<i32> = vec![1, 2, 3];

    //v1.iter().map(|x| x + 1); // does nothing! iterator returned by map is not used.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
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

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // can't use v1_iter after this call

    assert_eq!(total, 6);
}
