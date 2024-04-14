// 2549

fn main() {
    type_synonyms_1();
    type_synonyms_2();
    type_synonyms_3();
}

fn type_synonyms_1() {
    type Kilometers = i32;

    // The Kilometers type and i32 can be used interchangably.
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
}

fn type_synonyms_2() {
    // better use of type synonym is to reduce repetition. Make
    // a long type signature shorter.
    // look how many times I had to type that stuff.
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(_f: Box<dyn Fn() + Send + 'static>) {
        //snip
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // gotta return something
        Box::new(|| println!("hi"))
    }

    takes_long_type(f);

    let _fr = returns_long_type();
    // this doesn't do anything, but at least it compiles, and shows
    // how to do it the hard way.
}

fn type_synonyms_3() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(_f: Thunk) {
        //snip
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hi"))
    }

    takes_long_type(f);

    let _fr = returns_long_type();
}
