#[macro_use]
extern crate log;

use std::env;
use log::Level;
use pnet::datalink;
use pnet::EthernetPacket;
use pnet::packet;
use pnet::{ipv4_handler, ip6_handler};

fn main() {
    println!("Hello, world!");

    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        error!("Please specify target interface name");
        std::process::exit(1);
    }

    let interface_name = &args[1];

    //select interface
    let interfaces = datalink::interfaces();

    let interace = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
        .expect("Failed to get interface");


    // create packet capture
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()){

        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) =>("Failed to create datalink channel {}", e),
    };

    loop{
        match rx.next(){
            Ok(frame) => {
                // 受信データからイーサネットフレームの構築
                let frame = EthernetPacket::new(frame).unwrap();

                match frame.get_ethertype(){
                    EtherTypes::Ipv4 =>{
                        ipv4_handler(&frame);
                    }
                    EtherTypes::Ipv6 =>{
                        ipv6_handler(&frame);
                    }
                    _ => {
                        info!("Not an IPv4 or IPv6 packet");
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to read: {}", e);
        }
    }
}
