use std::net::Ipv4Addr;

fn main() {
    let ip = Ipv4Addr::new(192, 168, 40, 131);
    let mask = Ipv4Addr::new(255, 255, 255, 224);
    let and = ip & mask;
    println!("{and}");
}
