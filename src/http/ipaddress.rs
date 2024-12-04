use local_ip_address::list_afinet_netifas;
use async_std::net::IpAddr;

pub fn get_local_ip_address() -> Vec<(std::string::String, IpAddr)> {
    let network_interfaces = list_afinet_netifas().unwrap();
    return network_interfaces;
}