//! Simple HTTP server.
//!
//! Benchmark with [Apache HTTP server benchmarking tool](https://httpd.apache.org/docs/2.4/programs/ab.html):
//!
//! ```
//! ab -n 5000 -c 20 http://X.X.X.X:5555/
//! ```

extern crate alloc;

use std::str::FromStr;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::Result;
use std::thread;

const LOCAL_IP: &str = "10.0.2.15";
const LOCAL_PORT: u16 = 5555;

macro_rules! header {
    () => {
        "\
HTTP/1.1 200 OK\r\n\
Content-Type: text/html\r\n\
Content-Length: {}\r\n\
Connection: close\r\n\
\r\n\
{}"
    };
}

const CONTENT: &str = r#"<html>
<head>
  <title>Hello, ArceOS</title>
</head>
<body>
  <center>
    <h1>[STD]: Hello, <a href="https://github.com/rcore-os/arceos">ArceOS</a></h1>
  </center>
  <hr>
  <center>
    <i>Powered by <a href="https://github.com/rcore-os/arceos/tree/main/apps/net/httpserver">ArceOS example HTTP server</a> v0.1.0</i>
  </center>
</body>
</html>
"#;

fn http_server(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0u8; 1024];
    stream.read(&mut buf)?;

    let reponse = alloc::format!(header!(), CONTENT.len(), CONTENT);
    stream.write_all(reponse.as_bytes())?;

    Ok(())
}

fn accept_loop() -> Result<()> {
    let (addr, port) = (IpAddr::from_str(LOCAL_IP).unwrap(), LOCAL_PORT);
    let listener = TcpListener::bind::<(IpAddr, u16)>((addr, port).into())?;
    println!("listen on: http://{}/", listener.local_addr().unwrap());

    let mut i = 0;
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("new client {}: {}", i, addr);
                thread::spawn(move || match http_server(stream) {
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
    println!("[STD]: Hello, ArceOS HTTP server!");
    accept_loop().expect("test HTTP server failed");
}
