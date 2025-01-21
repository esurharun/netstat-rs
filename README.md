netstat-esr
==========


Cross-platform library to retrieve network sockets information.
Aims to be optimal by using low-level OS APIs instead of command line utilities.
Provides unified interface and returns data structures which may have additional fields depending on platform.

## Example

```rust
extern crate netstat;

use netstat::*;

fn main() {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();
    for si in sockets_info {
        println!("Active connection: {}",si);
    }
}
```

## Details

- On Windows, library uses [GetExtendedTcpTable](https://docs.microsoft.com/en-us/windows/desktop/api/iphlpapi/nf-iphlpapi-getextendedtcptable) & [GetExtendedUdpTable](https://docs.microsoft.com/en-us/windows/desktop/api/iphlpapi/nf-iphlpapi-getextendedudptable) (iphlpapi)
- On Linux, it uses [NETLINK_INET_DIAG](http://manpages.ubuntu.com/manpages/bionic/en/man7/sock_diag.7.html) protocol and performs pid lookup by traversing `procfs`
- On OS X, it uses sysctl system calls with "net.inet.tcp.pcblist" header to populate active connections. 
Got tech details from FreeBSD version of [netstat][https://web.mit.edu/freebsd/head/usr.bin/systat/netstat.c].
Pid lookup is not ready yet!


## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
