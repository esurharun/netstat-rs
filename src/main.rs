mod ffi;

use self::ffi::*;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Copy, Clone, Debug)]
pub enum Protocol {
    TCP,
    UDP,
}

#[derive(Copy, Clone, Debug)]
pub struct BindingInfo {
    pub protocol: Protocol,
    pub ip: IpAddr,
    pub port: u16,
    pub pid: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum ErrorType {
    InitializationError(u32),
    ErrorWithCode(u32),
}

#[derive(Copy, Clone, Debug)]
pub struct Error {
    pub method_name: &'static str,
    pub error_type: ErrorType,
}

fn get_extended_udp_table(bindings: &mut Vec<BindingInfo>) -> Result<(), Error> {
    unsafe {
        let mut buffer_size: DWORD = 0;
        let mut err_code = GetExtendedUdpTable(
            std::ptr::null_mut(),
            &mut buffer_size,
            FALSE,
            AF_INET,
            UDP_TABLE_OWNER_PID,
            0,
        );
        let mut buffer = Vec::<u8>::new();
        let mut iterations = 0;
        while err_code == ERROR_INSUFFICIENT_BUFFER {
            buffer = Vec::<u8>::with_capacity(buffer_size as usize);
            err_code = GetExtendedUdpTable(
                buffer.as_mut_ptr() as PVOID,
                &mut buffer_size,
                FALSE,
                AF_INET,
                UDP_TABLE_OWNER_PID,
                0,
            );
            iterations += 1;
            if iterations > 100 {
                return Result::Err(Error {
                    method_name: "GetExtendedUdpTable",
                    error_type: ErrorType::InitializationError(iterations),
                });
            }
        }
        if err_code == NO_ERROR {
            let table_ref = &*(buffer.as_ptr() as *const MIB_UDPTABLE_OWNER_PID);
            let rows_count = table_ref.rows_count as usize;
            let row_ptr = &table_ref.rows[0] as *const MIB_UDPROW_OWNER_PID;
            for i in 0..rows_count {
                let row = &*row_ptr.offset(i as isize);
                bindings.push(BindingInfo {
                    protocol: Protocol::UDP,
                    ip: IpAddr::V4(Ipv4Addr::from(u32::from_be(row.local_addr))),
                    port: u16::from_be(row.local_port as u16),
                    pid: row.owning_pid,
                });
            }
            return Result::Ok(());
        } else {
            return Result::Err(Error {
                method_name: "GetExtendedUdpTable",
                error_type: ErrorType::ErrorWithCode(err_code),
            });
        }
    }
}

fn main() {
    let mut bindings = Vec::<BindingInfo>::with_capacity(128);
    get_extended_udp_table(&mut bindings).expect("Error!!!");
    for binding in bindings {
        println!(
            "ip = {}, port = {}, pid = {}",
            binding.ip, binding.port, binding.pid
        );
    }
}
