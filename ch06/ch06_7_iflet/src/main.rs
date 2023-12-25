fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let un_conf: Option<u8> = Option::None;
    if let None = un_conf {
        println!("un_conf = {:?}", un_conf);
    }
}
