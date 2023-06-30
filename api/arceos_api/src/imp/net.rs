use axerrno::AxResult;
use axnet::TcpSocket;
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

pub struct AxTcpSocketHandle(TcpSocket);

const fn into_ax_ipaddr(ip: IpAddr) -> axnet::IpAddr {
    match ip {
        IpAddr::V4(ip) => axnet::IpAddr::Ipv4(axnet::Ipv4Addr(ip.octets())),
        _ => panic!("IPv6 not supported"),
    }
}

const fn into_core_ipaddr(ip: axnet::IpAddr) -> IpAddr {
    match ip {
        axnet::IpAddr::Ipv4(ip) => IpAddr::V4(unsafe { core::mem::transmute(ip.0) }),
    }
}

const fn into_ax_sockaddr(addr: SocketAddr) -> axnet::SocketAddr {
    axnet::SocketAddr::new(into_ax_ipaddr(addr.ip()), addr.port())
}

const fn into_core_sockaddr(addr: axnet::SocketAddr) -> SocketAddr {
    SocketAddr::new(into_core_ipaddr(addr.addr), addr.port)
}

pub fn ax_tcp_socket() -> AxTcpSocketHandle {
    AxTcpSocketHandle(TcpSocket::new())
}

pub fn ax_tcp_socket_addr(socket: &AxTcpSocketHandle) -> AxResult<SocketAddr> {
    socket.0.local_addr().map(into_core_sockaddr)
}

pub fn ax_tcp_peer_addr(socket: &AxTcpSocketHandle) -> AxResult<SocketAddr> {
    socket.0.peer_addr().map(into_core_sockaddr)
}

pub fn ax_tcp_connect(socket: &mut AxTcpSocketHandle, addr: SocketAddr) -> AxResult {
    socket.0.connect(into_ax_sockaddr(addr))
}

pub fn ax_tcp_bind(socket: &mut AxTcpSocketHandle, addr: SocketAddr) -> AxResult {
    socket.0.bind(into_ax_sockaddr(addr))
}

pub fn ax_tcp_listen(socket: &mut AxTcpSocketHandle, _backlog: usize) -> AxResult {
    socket.0.listen()
}

pub fn ax_tcp_accept(socket: &mut AxTcpSocketHandle) -> AxResult<(AxTcpSocketHandle, SocketAddr)> {
    let new_sock = socket.0.accept()?;
    let addr = new_sock.peer_addr().map(into_core_sockaddr)?;
    Ok((AxTcpSocketHandle(new_sock), addr))
}

pub fn ax_tcp_send(socket: &mut AxTcpSocketHandle, buf: &[u8]) -> AxResult<usize> {
    socket.0.send(buf)
}

pub fn ax_tcp_recv(socket: &mut AxTcpSocketHandle, buf: &mut [u8]) -> AxResult<usize> {
    socket.0.recv(buf)
}

pub fn ax_tcp_shutdown(socket: &AxTcpSocketHandle) -> AxResult {
    socket.0.shutdown()
}

pub fn ax_get_addr_info(domain_name: &str) -> AxResult<alloc::vec::Vec<IpAddr>> {
    Ok(axnet::resolve_socket_addr(domain_name)?
        .into_iter()
        .map(into_core_ipaddr)
        .collect())
}
