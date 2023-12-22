fn main() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(10, 0, 0, 0);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home ip addr: {:?}", home);
    println!("loopback ip addr: {:?}", loopback);
}

