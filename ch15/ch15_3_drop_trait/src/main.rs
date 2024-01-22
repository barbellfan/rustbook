struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    using_the_drop_trait();
    dropping_a_val_early();
}

fn using_the_drop_trait() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
}

fn dropping_a_val_early() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created.");
    //c.drop(); // can't call this directly
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.")
}
