
#[macro_use]
extern crate log;

use pnet::packet::tcp::TcpFlags;
use std::net::Ipv4Addr;
use std::fs;
use std::collections::HashMap;

struct PacketInfo {
    my_ipaddr: Ipv4Addr,
    target_ipaddr: Ipv4Addr,
    my_port: u16,
    maximum_port: u16,
    scan_type: ScanType,
}

#[derive(Copy, Clone)]
enum ScanType {
    Syn = TcpFlags::SYN as isize,
    Fin = TcpFlags::FIN as isize,
    Xmas = { TcpFlags::FIN | TcpFlags::URG | TcpFlags::PSH } as isize,
    Null = 0,
}

use env_logger;
use log::{error, warn, info, debug};
use std::env;
use std::process;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        error!("Bad number of arguments. [ippaddr] [scantype]");
        process::exit(1);
    }

    let packet_info = {
        let contents = fs::read_to_string("env.").expect("Failed to read env file");

        let lines: Vec<_> = contents.split('\n').collect();

        let mut map = HashMap::new();

        for line in lines {
            let elm: Vec<_> = line.split('=').map(str::trim).collect();
            if elm.len() == 2{
                map.insert(elm[0], elm[1]);
            }
        }

        PacketInfo {
            my_ipaddr: map["MY_IPADDR"].parse().expect("invalid ipaddr"),
            target_ipaddr: args[1].parse().expect("invalid target ipaddr"),
            my_port: map["MY_PORT"].parse().expect("invalid port number"),
            maximum_port: map["MAXIMUM_PORT_NUM"].parse().expect("invalid maximum port num"),
            scan_type: match args[2].as_str(){
                "sS" => ScanType::Syn,
                "sF" => ScanType::Fin,
                "sX" => ScanType::Xmas,
                "sN" => ScanType::Null,

                _=> {
                    error!("Undefined scan method, only accept [sS|sF|sX|sN]");
                    process::exit(1);
                }
            },
        }
    };
}
