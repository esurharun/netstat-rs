use integrations::osx::sysctlw::*;
use types::*;

/// Iterate through sockets information.
pub fn iterate_sockets_info(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
) -> Result<Vec<SocketInfo>, Error> {
    unsafe {
        return iterate_sysctl_info(af_flags, proto_flags);
    }
}
