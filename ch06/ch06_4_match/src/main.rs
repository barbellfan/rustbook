fn main() {
    let p = Coin::Penny;
    println!("p: {}", value_in_cents(p));

    let n = Coin::Nickel;
    println!("n: {}", value_in_cents(n));

    let d = Coin::Dime;
    println!("d: {}", value_in_cents(d));

    let q = Coin::Quarter;
    println!("q: {}", value_in_cents(q(UsState::Oregon)));
}

#[derive(Debug)]
#[allow(unused)]
enum UsState {
    Alabama,
    Alaska,
    // I'm not doing all of these
    Washington,
    Oregon,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
