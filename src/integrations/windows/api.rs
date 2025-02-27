use integrations::windows::ffi::*;
use integrations::windows::socket_table_iterator::SocketTableIterator;
use types::*;

/// Iterate through sockets information.
pub fn iterate_sockets_info(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
) -> Result<Vec<SocketInfo>, Error> {
    let ipv4 = af_flags.contains(AddressFamilyFlags::IPV4);
    let ipv6 = af_flags.contains(AddressFamilyFlags::IPV6);
    let tcp = proto_flags.contains(ProtocolFlags::TCP);
    let udp = proto_flags.contains(ProtocolFlags::UDP);
    let mut iterators = Vec::with_capacity(4);
    if ipv4 {
        if tcp {
            iterators.push(SocketTableIterator::new::<MIB_TCPTABLE_OWNER_PID>()?);
        }
        if udp {
            iterators.push(SocketTableIterator::new::<MIB_UDPTABLE_OWNER_PID>()?);
        }
    }
    if ipv6 {
        if tcp {
            iterators.push(SocketTableIterator::new::<MIB_TCP6TABLE_OWNER_PID>()?);
        }
        if udp {
            iterators.push(SocketTableIterator::new::<MIB_UDP6TABLE_OWNER_PID>()?);
        }
    }
    let mut ret: Vec<SocketInfo> = Vec::new();

    for it in iterators.into_iter().flatten() {
        let mut iter = it.iter();
        let mut item = iter.next();
        while !item.is_none() {
            ret.push(item.unwrap().clone());
            item = iter.next();
        }
    }

    Ok(ret)
}
