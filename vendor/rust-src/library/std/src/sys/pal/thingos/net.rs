
use crate::io;
use crate::net::{SocketAddr, Shutdown, Ipv4Addr, Ipv6Addr};
use crate::time::Duration;

pub struct TcpStream;
pub struct TcpListener;
pub struct UdpSocket;
pub struct LookupHost;

use crate::fmt;

impl TcpStream {
    pub fn connect(_addr: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
        Err(io::Error::UNSUPPORTED_PLATFORM)
    }
    pub fn connect_timeout(_addr: &SocketAddr, _timeout: Duration) -> io::Result<TcpStream> {
        Err(io::Error::UNSUPPORTED_PLATFORM)
    }
// ...

    pub fn set_read_timeout(&self, _dur: Option<Duration>) -> io::Result<()> { Ok(()) }
    pub fn set_write_timeout(&self, _dur: Option<Duration>) -> io::Result<()> { Ok(()) }
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> { Ok(None) }
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> { Ok(None) }
    pub fn peek(&self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> { Ok(0) }
    pub fn peer_addr(&self) -> io::Result<SocketAddr> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn socket_addr(&self) -> io::Result<SocketAddr> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn shutdown(&self, _how: Shutdown) -> io::Result<()> { Ok(()) }
    pub fn duplicate(&self) -> io::Result<TcpStream> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn set_nodelay(&self, _t: bool) -> io::Result<()> { Ok(()) }
    pub fn nodelay(&self) -> io::Result<bool> { Ok(false) }
    pub fn set_ttl(&self, _t: u32) -> io::Result<()> { Ok(()) }
    pub fn ttl(&self) -> io::Result<u32> { Ok(0) }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> { Ok(None) }
    pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> { Ok(()) }
}

impl TcpListener {
    pub fn bind(_addr: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
        Err(io::Error::UNSUPPORTED_PLATFORM)
    }
    pub fn socket_addr(&self) -> io::Result<SocketAddr> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn duplicate(&self) -> io::Result<TcpListener> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn set_ttl(&self, _ttl: u32) -> io::Result<()> { Ok(()) }
    pub fn ttl(&self) -> io::Result<u32> { Ok(0) }
    pub fn set_only_v6(&self, _only_v6: bool) -> io::Result<()> { Ok(()) }
    pub fn only_v6(&self) -> io::Result<bool> { Ok(false) }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> { Ok(None) }
    pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> { Ok(()) }
}

impl UdpSocket {
    pub fn bind(_addr: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
        Err(io::Error::UNSUPPORTED_PLATFORM)
    }
    pub fn peer_addr(&self) -> io::Result<SocketAddr> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn socket_addr(&self) -> io::Result<SocketAddr> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn recv_from(&self, _buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn peek_from(&self, _buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn send_to(&self, _buf: &[u8], _addr: &SocketAddr) -> io::Result<usize> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn duplicate(&self) -> io::Result<UdpSocket> { Err(io::Error::UNSUPPORTED_PLATFORM) }
    pub fn set_read_timeout(&self, _dur: Option<Duration>) -> io::Result<()> { Ok(()) }
    pub fn set_write_timeout(&self, _dur: Option<Duration>) -> io::Result<()> { Ok(()) }
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> { Ok(None) }
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> { Ok(None) }
    pub fn set_broadcast(&self, _broadcast: bool) -> io::Result<()> { Ok(()) }
    pub fn broadcast(&self) -> io::Result<bool> { Ok(false) }
    pub fn set_multicast_loop_v4(&self, _multicast_loop_v4: bool) -> io::Result<()> { Ok(()) }
    pub fn multicast_loop_v4(&self) -> io::Result<bool> { Ok(false) }
    pub fn set_multicast_ttl_v4(&self, _multicast_ttl_v4: u32) -> io::Result<()> { Ok(()) }
    pub fn multicast_ttl_v4(&self) -> io::Result<u32> { Ok(0) }
    pub fn set_multicast_loop_v6(&self, _multicast_loop_v6: bool) -> io::Result<()> { Ok(()) }
    pub fn multicast_loop_v6(&self) -> io::Result<bool> { Ok(false) }
    pub fn join_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> io::Result<()> { Ok(()) }
    pub fn leave_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> io::Result<()> { Ok(()) }
    pub fn join_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> io::Result<()> { Ok(()) }
    pub fn leave_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> io::Result<()> { Ok(()) }
    pub fn set_ttl(&self, _ttl: u32) -> io::Result<()> { Ok(()) }
    pub fn ttl(&self) -> io::Result<u32> { Ok(0) }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> { Ok(None) }
    pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> { Ok(()) }
    pub fn recv(&self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    pub fn peek(&self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    pub fn send(&self, _buf: &[u8]) -> io::Result<usize> { Ok(0) }
    pub fn connect(&self, _addr: &SocketAddr) -> io::Result<()> { Ok(()) }

    pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UdpSocket").finish()
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> { None }
}

pub fn lookup_host(_host: &str) -> io::Result<LookupHost> {
    Err(io::Error::UNSUPPORTED_PLATFORM)
}
