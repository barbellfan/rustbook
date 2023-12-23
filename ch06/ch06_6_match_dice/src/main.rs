fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn move_player(num_spaces: i32) {
    println!("move player {} spaces", num_spaces);
}

fn remove_fancy_hat() {
    println!("remove fancy hat");
}

fn add_fancy_hat() {
    println!("add fancy hat");
}
