use core::str::FromStr;

use std::io::{Result, Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::thread;

const LOCAL_IP: &str = "10.0.2.15";
const LOCAL_PORT: u16 = 5555;

fn reverse(buf: &[u8]) -> Vec<u8> {
    let mut lines = buf
        .split(|&b| b == b'\n')
        .map(Vec::from)
        .collect::<Vec<_>>();
    for line in lines.iter_mut() {
        line.reverse();
    }
    lines.join(&b'\n')
}

fn echo_server(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0u8; 1024];
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            return Ok(());
        }
        //println!("before write_all...");
        stream.write_all(reverse(&buf[..n]).as_slice())?;
        //println!("after write_all ok!");
    }
}

fn accept_loop() -> Result<()> {
    println!("### accept_loop ...");
    let (addr, port) = (IpAddr::from_str(LOCAL_IP).unwrap(), LOCAL_PORT);
    println!("### listen ...");
    let listener = TcpListener::bind::<(IpAddr, u16)>((addr, port).into())?;
    println!("listen on: {}", listener.local_addr().unwrap());

    let mut i = 0;
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("new client {}: {}", i, addr);
                thread::spawn(move || match echo_server(stream) {
                    Err(e) => println!("client connection error: {:?}", e),
                    Ok(()) => println!("client {} closed successfully", i),
                });
            }
            Err(e) => return Err(e),
        }
        i += 1;
    }
}

fn main() {
    println!("Hello, echo server!");
    accept_loop().expect("test echo server failed");
}
