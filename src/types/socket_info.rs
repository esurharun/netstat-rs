use std::net::IpAddr;
use types::tcp_state::TcpState;

/// General socket information.
#[derive(Clone, Debug)]
pub struct SocketInfo {
    /// Protocol-specific socket information.
    pub protocol_socket_info: ProtocolSocketInfo,
    /// Identifiers of processes associated with this socket.
    pub associated_pids: Vec<u32>,
    #[cfg(target_os = "linux")]
    pub inode: u32,
}

impl std::fmt::Display for SocketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_info) => write!(
                f,
                "TCP: {}:{} {}:{} {}",
                tcp_info.local_addr,
                tcp_info.local_port,
                tcp_info.remote_addr,
                tcp_info.remote_port,
                tcp_info.state
            ),
            ProtocolSocketInfo::Udp(udp_info) => write!(
                f,
                "UDP: {}:{} {}:{} ",
                udp_info.local_addr,
                udp_info.local_port,
                udp_info.remote_addr,
                udp_info.remote_port
            ),
        }
    }
}

impl SocketInfo {
    /// Checks whether this socket info describes a TCP socket.
    pub fn is_tcp(&self) -> bool {
        match self.protocol_socket_info {
            ProtocolSocketInfo::Tcp(_) => true,
            _ => false,
        }
    }
    /// Checks whether this socket info describes an UDP socket.
    pub fn is_udp(&self) -> bool {
        match self.protocol_socket_info {
            ProtocolSocketInfo::Udp(_) => true,
            _ => false,
        }
    }
}

impl PartialEq<SocketInfo> for SocketInfo {
    fn eq(&self, other: &SocketInfo) -> bool {
        return self == other;
    }
}

/// Protocol-specific socket information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProtocolSocketInfo {
    /// TCP-specific socket information.
    Tcp(TcpSocketInfo),
    /// UDP-specific socket information.
    Udp(UdpSocketInfo),
}

/// TCP-specific socket information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TcpSocketInfo {
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_port: u16,
    pub state: TcpState,
}

/// UDP-specific socket information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UdpSocketInfo {
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_port: u16,
}
