use super::{SocketAddr, ToSocketAddrs};
use crate::io::{self, prelude::*};
use arceos_api::net::{self as api, AxTcpSocketHandle};

/// A TCP stream between a local and a remote socket.
pub struct TcpStream(AxTcpSocketHandle);

/// A TCP socket server, listening for connections.
pub struct TcpListener(AxTcpSocketHandle);

impl TcpStream {
    /// Opens a TCP connection to a remote host.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        super::each_addr(addr, |addr: io::Result<&SocketAddr>| {
            let addr = addr?;
            let socket = api::ax_tcp_socket();
            api::ax_tcp_connect(&socket, *addr)?;
            Ok(TcpStream(socket))
        })
    }

    /// Returns the socket address of the local half of this TCP connection.
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        api::ax_tcp_socket_addr(&self.0)
    }

    /// Returns the socket address of the remote peer of this TCP connection.
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        api::ax_tcp_peer_addr(&self.0)
    }

    /// Shuts down the connection.
    pub fn shutdown(&self) -> io::Result<()> {
        api::ax_tcp_shutdown(&self.0)
    }
}

impl Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        api::ax_tcp_recv(&self.0, buf)
    }
}

impl Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        api::ax_tcp_send(&self.0, buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl TcpListener {
    /// Creates a new `TcpListener` which will be bound to the specified
    /// address.
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
        super::each_addr(addr, |addr: io::Result<&SocketAddr>| {
            let addr = addr?;
            let backlog = 128;
            let socket = api::ax_tcp_socket();
            api::ax_tcp_bind(&socket, *addr)?;
            api::ax_tcp_listen(&socket, backlog)?;
            Ok(TcpListener(socket))
        })
    }

    /// Returns the local socket address of this listener.
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        api::ax_tcp_socket_addr(&self.0)
    }

    /// Accept a new incoming connection from this listener.
    ///
    /// This function will block the calling thread until a new TCP connection
    /// is established. When established, the corresponding [`TcpStream`] and the
    /// remote peer's address will be returned.
    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        api::ax_tcp_accept(&self.0).map(|(a, b)| (TcpStream(a), b))
    }
}
