
#[macro_use]
extern crate log;

use pnet::packet::tcp::TcpFlags;
use std::net::Ipv4Addr;
use std::fs;
use std::collections::HashMap;
use std::any::type_name;
use pnet::*;
use pnet::transport::*;
use pnet::packet::ip::*;
use pnet::packet::tcp::*;
use pnet::packet::*;


const TCP_SIZE: usize = 1024;

fn print_typename<T>(_: &T){
    println!("{}", std::any::type_name::<T>());
}

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

    print_typename(&args);

    if args.len() != 3 {
        error!("Bad number of arguments. [ippaddr] [scantype]");
        process::exit(1);
    }



    let packet_info = {
        let contents = fs::read_to_string(".env").expect("Failed to read env file");

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

    println!("open!!");

    //トランスポート層のチャンネルを開く
    let (mut ts, mut tr) = transport::transport_channel(
        1024,
        TransportChannelType::Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Tcp)),
    ).expect("Failed to open channel");

    reyon::join( 
        || send_packet( &mut ts, &packet_info),
        || receive_packets(&mut tr, &packet_info),
        || receive_packets(&mut tr, &packet_info),
    );
}


fn build_packet(packet_info: &PacketInfo) -> [u8; TCP_SIZE]{
    // build TCP Header
    let mut tcp_buffer = [0u8; TCP_SIZE];
    let mut tcp_header = MutableTcpPacket::new(&mut tcp_buffer[..]).unwrap();
    tcp_header.set_source(packet_info.my_port);

    // オプションを含まないので20オクテットまでがTCOヘッダ．4オクテット単位で指定する
    tcp_header.set_data_offset(5);
    tcp_header.set_flags(packet_info.scan_type as u16);

    let checksum = tcp::ipv4_checksum(
        &tcp_header.to_immutable(),
        &packet_info.my_ipaddr,
        &packet_info.target_ipaddr,
    );
    tcp_header.set_checksum(checksum);

    tcp_buffer
}


//send
fn reregister_destination_port(
    target: u16,
    tcp_header: &mut MutableTcpPacket,
    packet_info: &PacketInfo
){
    tcp_header.set_destination(target);
    let checksum = tcp::ipv4_checksum(
        &tcp_header.to_immutable(),
        &packet_info.my_ipaddr,
        &packet_info.target_ipaddr,
    );
    tcp_header.set_checksum(checksum);
}

//receive
fn receive_packets(
    tr: & mut TransportReceiver,
    packet_info: &PacketInfo,
) -> Result<(), failure::Error>{

    let mut reply_ports = Vec::new();
    let mut packet_iter = transport::tcp_packet_iter(tr);

    loop {

        let tcp_packet = match packet_iter.next(){
            Ok((tcp_packet, _)) => {
                if tcp_packet.get_destination() == packet_info.my_port {
                    tcp_packet
                } else{
                    continue;
                }
            }
            Err(_) => continue,
        };


        let target_port = tcp_packet.get_source();

        match packet_info.scan_type {
            ScanType::Syn => {
                if tcp_packet.get_flags() == TcpFlags::SYN | TcpFlags::ACK {
                    println!("port {} is open", target_port);
                }
            },
            // SYNスキャン以外はレスポンスが帰ってきたポートを記録
            ScanType::Fin | ScanType::Xmas | ScanType::Null => {
              reply_ports.push(target_port);  
            },
        }

        if target_port != packet_info.maximum_port {
            continue;
        }

        match packet_info.scan_type {
            ScanType::Fin | ScanType::Xmas | ScanType::Null => {
                for i in 1..=packet_info.maximum_port{

                    if reply_ports.iter().find(|&&x|x==i).is_none(){
                        println!("port {} is open", i);
                    }
                }
            }
            _=>{}
        }
        //処理の終了
        return Ok(());
    }
}


