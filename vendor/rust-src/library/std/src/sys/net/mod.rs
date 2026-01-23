/// This module contains the implementations of `TcpStream`, `TcpListener` and
/// `UdpSocket` as well as related functionality like DNS resolving.
#[cfg(target_os = "thingos")]
    mod thingos;
    #[cfg(target_os = "thingos")]
    use thingos as imp;

    mod connection;
    pub use connection::*;

mod hostname;
pub use hostname::hostname;
