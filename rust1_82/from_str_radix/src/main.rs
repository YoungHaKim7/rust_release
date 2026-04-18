fn main() {
    println!("Hello, world!");
    let str_string = "15";

    let res = i64::from_str_radix(str_string, 16);
    let res08 = i64::from_str_radix(str_string, 8);

    println!("res 15 : {res:?}");
    println!("res 15 : {res08:?}");

    // Memory address from log
    let log_line = "Program loaded at address 0x7ffd1234";
    let hex_str = log_line.split("0x").last().unwrap();
    let addr = usize::from_str_radix(hex_str, 16).unwrap();
    println!("Memory address: {addr:#x} (decimal: {addr})");
}
