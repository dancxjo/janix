//! DHCPv4 client using smoltcp

use smoltcp::iface::Interface;
use smoltcp::socket::dhcpv4::{Socket as Dhcpv4Socket, Event};
use smoltcp::time::Duration;
use smoltcp::wire::Ipv4Address;

use crate::ipc_device::IpcNicDevice;

#[derive(Debug)]
#[allow(dead_code)]
pub enum DhcpError {
    Timeout,
    Failed,
}

pub struct DhcpConfig {
    pub ip: Ipv4Address,
    pub gateway: Ipv4Address,
    pub dns: Ipv4Address,
}

pub fn run_dhcp(
    iface: &mut Interface,
    device: &mut IpcNicDevice,
) -> Result<DhcpConfig, DhcpError> {
    let mut sockets_storage: [smoltcp::iface::SocketStorage; 1] = Default::default();
    let mut socket_set = smoltcp::iface::SocketSet::new(&mut sockets_storage[..]);

    let dhcp_socket = Dhcpv4Socket::new();
    let dhcp_handle = socket_set.add(dhcp_socket);

    stem::info!("DHCP: Starting discovery...");

    let start = IpcNicDevice::now();
    let timeout = start + Duration::from_secs(30);

    loop {
        let now = IpcNicDevice::now();
        if now > timeout {
            return Err(DhcpError::Timeout);
        }

        iface.poll(now, device, &mut socket_set);

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

                    // Apply configuration to interface
                    iface.update_ip_addrs(|addrs| {
                        addrs.clear();
                        addrs.push(smoltcp::wire::IpCidr::Ipv4(config.address)).ok();
                    });

                    if let Some(route) = config.router {
                        iface
                            .routes_mut()
                            .add_default_ipv4_route(route)
                            .ok();
                    }

                    return Ok(DhcpConfig { ip, gateway, dns });
                }
                Event::Deconfigured => {
                    stem::warn!("DHCP: Deconfigured");
                }
            }
        }

        stem::time::sleep_ms(10);
    }
}
