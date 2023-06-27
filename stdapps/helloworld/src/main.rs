use std::os::arceos::abi;

pub fn main() {
    println!("[Rust-STD]: Hello, world!");
    unsafe {
        println!("{:?}", abi::sys_getcwd());
        abi::sys_console_write_bytes(b"From arceos_api...\n");
        abi::sys_terminate();
    }
}
