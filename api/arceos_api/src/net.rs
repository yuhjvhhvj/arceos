//
// Socket stuff
//
use core::str::FromStr;
use core::net::{SocketAddr as StdSocketAddr, IpAddr};
use alloc::string::ToString;
use alloc::boxed::Box;
use axbase::{AF_UNSPEC, SOCK_STREAM, SOCK_DGRAM};
use axnet::TcpSocket;
use axnet::UdpSocket;
use axbase::AxError;
use libax::net::SocketAddr;

enum StdSocketWrap {
    Tcp(TcpSocket),
    Udp(UdpSocket),
}

fn sockaddr_std_to_ax(addr: &StdSocketAddr) -> SocketAddr {
    let s = addr.ip().to_string();
    let s = libax::net::IpAddr::from_str(&s).unwrap();
    SocketAddr::new(s, addr.port())
}

fn sockaddr_ax_to_std(addr: &SocketAddr) -> StdSocketAddr {
    let s = addr.addr.to_string();
    let s = IpAddr::from_str(&s).unwrap();
    StdSocketAddr::new(s, addr.port)
}

#[no_mangle]
pub fn sys_socket(family: i32, ty: i32) -> usize {
    assert!(family == AF_UNSPEC, "bad family {}", family);

    let sock = match ty {
        SOCK_STREAM => {
            libax::println!("sys_socket: tcp");
            StdSocketWrap::Tcp(TcpSocket::new())
        },
        SOCK_DGRAM => {
            libax::println!("sys_socket: udp");
            StdSocketWrap::Udp(UdpSocket::new())
        },
        _ => {
            panic!("bad socket type '{}'.", ty);
        }
    };
    let ptr = Box::leak(Box::new(sock));
    ptr as *mut _ as usize
}

#[no_mangle]
pub fn sys_bind(s: usize, addr: &StdSocketAddr) {
    let addr = sockaddr_std_to_ax(addr);

    let f = s as *mut StdSocketWrap;
    let wrap = unsafe { f.as_mut().unwrap() };
    match wrap {
        StdSocketWrap::Tcp(sock) => {
            libax::println!("sys_bind: tcp {:?}", addr);
            let _ = sock.bind(addr);
        },
        StdSocketWrap::Udp(sock) => {
            let _ = sock.bind(addr);
        },
    }
}

/// listen for connections on a socket
///
/// The `backlog` parameter defines the maximum length for the queue of pending
/// connections. Currently, the `backlog` must be one.
#[no_mangle]
pub fn sys_listen(s: usize, _backlog: i32) -> i32 {
    let f = s as *mut StdSocketWrap;
    let wrap = unsafe { f.as_mut().unwrap() };
    match wrap {
        StdSocketWrap::Tcp(sock) => {
            libax::println!("sys_listen: ");
            let _ = sock.listen();
            0
        },
        StdSocketWrap::Udp(_) => {
            panic!("sys_listen: udp");
        },
    }
}

#[no_mangle]
pub fn sys_getsockname(s: usize) -> Result<StdSocketAddr, AxError> {
    let f = s as *mut StdSocketWrap;
    let wrap = unsafe { f.as_mut().unwrap() };
    match wrap {
        StdSocketWrap::Tcp(sock) => {
            let ret = sock.local_addr()?;
            Ok(sockaddr_ax_to_std(&ret))
        },
        StdSocketWrap::Udp(sock) => {
            let ret = sock.local_addr()?;
            Ok(sockaddr_ax_to_std(&ret))
        },
    }
}

#[no_mangle]
pub fn sys_accept(s: usize) -> Result<(usize, StdSocketAddr), AxError> {
    let f = s as *mut StdSocketWrap;
    let wrap = unsafe { f.as_mut().unwrap() };
    match wrap {
        StdSocketWrap::Tcp(sock) => {
            libax::println!("sys_accept: ");
            let sock = sock.accept()?;
            let addr = sock.peer_addr()?;
            let addr = sockaddr_ax_to_std(&addr);
            let sock = StdSocketWrap::Tcp(sock);
            libax::println!("sys_accept: {:?}", addr);
            let ptr = Box::leak(Box::new(sock));
            Ok((ptr as *mut _ as usize, addr))
        },
        StdSocketWrap::Udp(_) => {
            panic!("sys_accept: udp");
        },
    }
}

#[no_mangle]
pub fn sys_recv(s: usize, buf: &mut [u8], _flags: i32) -> usize
{
    libax::println!("sys_recv: ...");
    let f = s as *mut StdSocketWrap;
    let wrap = unsafe { f.as_mut().unwrap() };
    match wrap {
        StdSocketWrap::Tcp(sock) => {
            libax::println!("sys_recv: tcp");
            let ret = sock.recv(buf).unwrap();
            libax::println!("sys_recv: ret{}", ret);
            ret
        },
        StdSocketWrap::Udp(_) => {
            panic!("sys_read: ");
        },
    }
}

#[no_mangle]
pub fn sys_send(s: usize, buf: &[u8]) -> usize {
    libax::println!("sys_send: ...");
    let f = s as *mut StdSocketWrap;
    let wrap = unsafe { f.as_mut().unwrap() };
    match wrap {
        StdSocketWrap::Tcp(sock) => {
            libax::println!("sys_send: ...");
            let ret = sock.send(buf).unwrap();
            libax::println!("sys_send: ok! ret {}", ret);
            ret
        },
        StdSocketWrap::Udp(_) => {
            panic!("sys_send: ");
        },
    }
}

#[no_mangle]
pub fn sys_connect(s: usize, addr: &StdSocketAddr) {
    let addr = sockaddr_std_to_ax(addr);

    let f = s as *mut StdSocketWrap;
    let wrap = unsafe { f.as_mut().unwrap() };
    match wrap {
        StdSocketWrap::Tcp(sock) => {
            libax::println!("sys_connect {:?}", addr);
            sock.connect(addr).unwrap()
        },
        StdSocketWrap::Udp(_) => {
            panic!("sys_connect: ");
        },
    }
}

#[no_mangle]
pub fn sys_recvfrom(s: usize, buf: &mut [u8], _flags: i32)
    -> (usize, StdSocketAddr)
{
    let f = s as *mut StdSocketWrap;
    let wrap = unsafe { f.as_mut().unwrap() };
    let (num, addr) = match wrap {
        StdSocketWrap::Tcp(_) => {
            panic!("sys_recvfrom: ");
        },
        StdSocketWrap::Udp(sock) => {
            sock.recv_from(buf).unwrap()
        },
    };
    let addr = sockaddr_ax_to_std(&addr);
    (num, addr)
}

#[no_mangle]
pub fn sys_sendto(s: usize, buf: &[u8], dst: &StdSocketAddr) -> usize {
    let dst = sockaddr_std_to_ax(dst);

    let f = s as *mut StdSocketWrap;
    let wrap = unsafe { f.as_mut().unwrap() };
    match wrap {
        StdSocketWrap::Tcp(_) => {
            panic!("sys_sendto: ");
        },
        StdSocketWrap::Udp(sock) => {
            sock.send_to(buf, dst).unwrap()
        },
    }
}

#[no_mangle]
pub fn sys_getaddrinfo(name: &str, port: u16)
    -> Result<alloc::vec::Vec<StdSocketAddr>, AxError>
{
    let mut ret: alloc::vec::Vec<StdSocketAddr> = alloc::vec![];
    let ips = axnet::resolve_socket_addr(name).unwrap();
    for ip in &ips {
        let s: SocketAddr = SocketAddr::new(*ip, port);
        let s = sockaddr_ax_to_std(&s);
        ret.push(s);
    }
    Ok(ret)
}

#[no_mangle]
pub fn sys_close_socket(handle: usize) {
    unsafe { core::ptr::drop_in_place(handle as *mut StdSocketWrap) }
}
