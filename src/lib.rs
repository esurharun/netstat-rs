//! Cross-platform library to retrieve network sockets information.
//! Tries to be optimal by using low-level OS APIs instead of command line utilities.
//! Provides unified interface and returns data structures which may have additional fields depending on platform.

#![allow(non_camel_case_types)]

#[macro_use]
extern crate bitflags;
extern crate libc;

mod integrations;
mod types;
mod utils;

pub use integrations::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_is_ok_for_any_flags() {
        let af_flags_combs = (0..AddressFamilyFlags::all().bits() + 1)
            .filter_map(|x| AddressFamilyFlags::from_bits(x))
            .collect::<Vec<AddressFamilyFlags>>();
        let proto_flags_combs = (0..ProtocolFlags::all().bits() + 1)
            .filter_map(|x| ProtocolFlags::from_bits(x))
            .collect::<Vec<ProtocolFlags>>();
        for af_flags in af_flags_combs.iter() {
            for proto_flags in proto_flags_combs.iter() {
                assert!(get_sockets_info(*af_flags, *proto_flags).is_ok());
            }
        }
    }

    #[test]
    fn result_is_empty_for_empty_flags() {

        let sockets_info =
            get_sockets_info(AddressFamilyFlags::empty(), ProtocolFlags::empty()).unwrap();
        assert!(sockets_info.len() == 0);
    }

    
    #[test]
    fn every_socket_is_either_tcp_or_udp() {

        let af_flags = AddressFamilyFlags::all();
        let proto_flags = ProtocolFlags::all();
        let sockets_info = iterate_sockets_info(af_flags, proto_flags).unwrap();

        for socket_info in sockets_info {
            assert!(socket_info.is_tcp() == !socket_info.is_udp());
        }
    }
}
