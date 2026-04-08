//! DHCPv4 client using smoltcp

use smoltcp::iface::Interface;
use smoltcp::phy::Device;
use smoltcp::socket::dhcpv4::{Event, Socket as Dhcpv4Socket};
use smoltcp::time::{Duration, Instant};
use smoltcp::wire::Ipv4Address;

<<<<<<< HEAD
=======
use crate::vfs_device::VfsNicDevice;

>>>>>>> 6225493e (feat(netd): replace IPC driver discovery with VFS file I/O on /dev/net/virtio0)
#[derive(Debug)]
#[allow(dead_code)]
pub enum DhcpError {
    Timeout,
    Failed,
}

pub struct DhcpConfig {
    pub ip: Ipv4Address,
    pub prefix_len: u8,
    pub gateway: Ipv4Address,
    pub dns: Ipv4Address,
}

<<<<<<< HEAD
fn now() -> Instant {
    Instant::from_millis(stem::time::now().as_millis() as i64)
}

pub fn run_dhcp<D: Device>(iface: &mut Interface, device: &mut D) -> Result<DhcpConfig, DhcpError> {
=======
pub fn run_dhcp(iface: &mut Interface, device: &mut VfsNicDevice) -> Result<DhcpConfig, DhcpError> {
>>>>>>> 6225493e (feat(netd): replace IPC driver discovery with VFS file I/O on /dev/net/virtio0)
    let mut sockets_storage: [smoltcp::iface::SocketStorage; 1] = Default::default();
    let mut socket_set = smoltcp::iface::SocketSet::new(&mut sockets_storage[..]);

    let dhcp_socket = Dhcpv4Socket::new();
    let dhcp_handle = socket_set.add(dhcp_socket);

    stem::info!("DHCP: Starting discovery...");

<<<<<<< HEAD
    let start = now();
    let timeout = start + Duration::from_secs(30);

    loop {
        let ts = now();
        if ts > timeout {
=======
    let start = VfsNicDevice::now();
    let timeout = start + Duration::from_secs(30);

    loop {
        let now = VfsNicDevice::now();
        if now > timeout {
>>>>>>> 6225493e (feat(netd): replace IPC driver discovery with VFS file I/O on /dev/net/virtio0)
            return Err(DhcpError::Timeout);
        }

        iface.poll(ts, device, &mut socket_set);

        let dhcp_socket = socket_set.get_mut::<Dhcpv4Socket>(dhcp_handle);

        if let Some(event) = dhcp_socket.poll() {
            match event {
                Event::Configured(config) => {
                    stem::info!("DHCP: Configuration received");

                    let ip = config.address.address();
                    let gateway = config.router.unwrap_or(Ipv4Address::UNSPECIFIED);
                    let dns = config
                        .dns_servers
                        .first()
                        .copied()
                        .unwrap_or(Ipv4Address::UNSPECIFIED);
                    let prefix_len = config.address.prefix_len();

                    // Apply configuration to interface
                    iface.update_ip_addrs(|addrs| {
                        addrs.clear();
                        addrs.push(smoltcp::wire::IpCidr::Ipv4(config.address)).ok();
                    });

                    if let Some(route) = config.router {
                        iface.routes_mut().add_default_ipv4_route(route).ok();
                    }

                    return Ok(DhcpConfig {
                        ip,
                        prefix_len,
                        gateway,
                        dns,
                    });
                }
                Event::Deconfigured => {
                    stem::warn!("DHCP: Deconfigured");
                }
            }
        }

<<<<<<< HEAD
        let delay = iface.poll_delay(ts, &socket_set);
        let wait_ms = delay.map(|d| d.total_millis()).unwrap_or(10).min(10);
=======
        let delay = iface.poll_delay(now, &socket_set);
        let wait_ms = delay.map(|d| d.total_millis()).unwrap_or(100).min(100);

>>>>>>> 6225493e (feat(netd): replace IPC driver discovery with VFS file I/O on /dev/net/virtio0)
        stem::time::sleep_ms(wait_ms as u64);
    }
}
