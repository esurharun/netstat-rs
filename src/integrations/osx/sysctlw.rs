use std::net::IpAddr;
use types::*;

use std::ffi::CStr;

extern "C" {
    fn get_connections(pr_flag: u8, af_flag: u8) -> *mut i8;
    fn free_get_connections(str: *mut i8);
}

pub unsafe fn iterate_sysctl_info(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
) -> Result<Vec<SocketInfo>, Error> {
    let mut ret = vec![];

    if proto_flags.is_empty() || af_flags.is_empty() {
        return Ok(ret);
    }

    // 0 -> IPv4
    // 1 -> IPv6
    // 2 -> Both
    let mut af_flag: u8 = 1;
    // 0 -> TCP
    // 1 -> UDP
    // 2 -> Both
    let mut pr_flag: u8 = 1;

    if af_flags.is_all() {
        af_flag = 2;
    } else if af_flags.contains(AddressFamilyFlags::IPV4) {
        af_flag = 0;
    }

    if proto_flags.is_all() {
        pr_flag = 2;
    } else if proto_flags.contains(ProtocolFlags::TCP) {
        pr_flag = 0;
    }

    unsafe {
        // Call the C function with 5 iterations
        let c_string = get_connections(pr_flag, af_flag);
        if c_string.is_null() {
            eprintln!("Failed to create the expanding string from C");
            return Ok(ret);
        }

        // Convert the C string to a Rust string
        let rust_string = CStr::from_ptr(c_string)
            .to_str()
            .expect("Failed to convert C string to Rust string");

        let results: Vec<&str> = rust_string.split("\t").map(|s| s.trim()).collect();

        let mut idx = 0;
        while idx < results.len() - 1 {
            let local_addr: Option<IpAddr> = parse_ip(results[idx]);
            let local_port: Option<u16> = parse_port(results[idx + 1]);
            let remote_addr: Option<IpAddr> = parse_ip(results[idx + 2]);
            let remote_port: Option<u16> = parse_port(results[idx + 3]);
            let state: TcpState = TcpState::from(results[idx + 4]);

            if local_addr != None
                && local_port != None
                && remote_addr != None
                && remote_port != None
            {
                let si = SocketInfo {
                    protocol_socket_info: ProtocolSocketInfo::Tcp(TcpSocketInfo {
                        local_addr: local_addr.unwrap(),
                        local_port: local_port.unwrap(),
                        remote_addr: remote_addr.unwrap(),
                        remote_port: remote_port.unwrap(),
                        state: state,
                    }),
                    associated_pids: Vec::with_capacity(0),
                };

                ret.push(si);
            }
            idx += 5;
        }

        // Free the memory allocated by the C code
        free_get_connections(c_string);
    }

    Ok(ret)
}

unsafe fn parse_port(s_port: &str) -> Option<u16> {
    match s_port.parse::<u16>() {
        Ok(port) => return Some(port),
        Err(_) => {
            return None;
        }
    }
}

unsafe fn parse_ip(s_ip: &str) -> Option<IpAddr> {
    match s_ip.parse::<IpAddr>() {
        Ok(ip) => return Some(ip),
        Err(_) => {
            return None;
        }
    }
}
