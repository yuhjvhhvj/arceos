// You can run this example from the root of the mio repo:
// make A=apps/std/mio-tcpserver STD=y NET=y run

use mio::net::TcpListener;
use std::io::{self, Read, Write};
use std::str::from_utf8;

// Some data we'll send over the connection.
const DATA: &[u8] = b"Hello world!\n";

#[cfg(not(target_os = "wasi"))]
fn main() -> io::Result<()> {
    env_logger::init();

    // Setup the TCP server socket.
    let addr = "127.0.0.1:5555".parse().unwrap();
    let server = TcpListener::bind(addr)?;

    println!("You can connect to the server using `nc`:");
    println!(" $ nc {} {}", addr.ip(), addr.port());
    println!("You'll see our welcome message and anything you type will be printed here.");

    let mut received_data = vec![0; 4096];
    loop {
        let (mut connection, address) = server.accept()?;

        println!("Accepted connection from: {}", address);
        connection.write_all(DATA)?;

        loop {
            let n = connection.read(&mut received_data)?;
            if n == 0 {
                break;
            }

            let received_data = &received_data[..n];
            if let Ok(str_buf) = from_utf8(received_data) {
                println!("Received data: {}", str_buf.trim_end());
                connection.write_all(received_data)?;
            } else {
                println!("Received (none UTF-8) data: {:?}", received_data);
            }
        }
    }
}
