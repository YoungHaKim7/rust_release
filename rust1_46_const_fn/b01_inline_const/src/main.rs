use std::net::Ipv6Addr;

fn mock_ip(use_localhost: bool) -> &'static Ipv6Addr {
    if use_localhost {
        &Ipv6Addr::LOCALHOST
    } else {
        const { &Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0) }
    }
}

const MMIO_BIT1: u8 = 4;
const MMIO_BIT2: u8 = 5;

fn main() {
    match read_mmio() {
        0 => {}
        const { 1 << MMIO_BIT1 } => println!("FOO"),
        const { 1 << MMIO_BIT2 } => println!("BAR"),

        _ => unreachable!(),
    }
}
