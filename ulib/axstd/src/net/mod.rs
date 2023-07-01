//! Networking primitives for TCP/UDP communication.

mod socket_addr;
mod tcp;
// mod udp;

pub use self::socket_addr::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs};
pub use self::tcp::{TcpListener, TcpStream};

use crate::io;

fn each_addr<A: ToSocketAddrs, F, T>(addr: A, mut f: F) -> io::Result<T>
where
    F: FnMut(io::Result<&SocketAddr>) -> io::Result<T>,
{
    let addrs = match addr.to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(e) => return f(Err(e)),
    };
    let mut last_err = None;
    for addr in addrs {
        match f(Ok(&addr)) {
            Ok(l) => return Ok(l),
            Err(e) => last_err = Some(e),
        }
    }
    Err(last_err.unwrap_or_else(|| {
        axerrno::ax_err_type!(InvalidInput, "could not resolve to any addresses")
    }))
}
