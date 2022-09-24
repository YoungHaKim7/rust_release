use std::{
    mem,
    net::{Ipv4Addr, SocketAddrV4},
};

fn main() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);

    println!("only {} bytes! ", mem::size_of_val(&addr));
}
