#[macro_use]
extern crate log;

use log::Level;
use pnet::datalink;
use pnet::packet;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use std::env;



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

    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
        .expect("Failed to get interface");


    // create packet capture
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()){

        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!( "Failed to create datalink channel {}", e),
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
        Err(e) => {
            error!("Failed to read: {}", e);
        }
    }
}
}

fn ipv4_handler(ethernet: & EthernetPacket){
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()){
        match packet.get_next_level_protocol(){
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            }
            _ =>{
                info!("Not a TCP or UDP packet");
            }
        }
    }
}


fn ipv6_handler(ethernet: &EthernetPacket){
if let Some(packet) = Ipv6Packet::new(ethernet.payload()){
        match packet.get_next_header(){
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            }
            _ =>{
                info!("Not a TCP or UDP packet");
            }
        }
    }
}


fn tcp_handler(packet: & GettableEndPoints){
    let tcp = TcpPacket::new(packet.get_payload());
    if let Some(tcp) = tcp{
        print_packet_info(packet, &tcp, "TCP");
    }
}


fn udp_handler(packet: & GettableEndPoints){
    let udp = UdpPacket::new(packet.get_payload());
    if let Some(udp) = udp{
        print_packet_info(packet, &udp, "UDP");
    }
}

fn print_packet_info(l3: &GettableEndPoints, l4: &GettableEndPoints, proto: &str
)
{
    println!(
        "Captureed a {} packet from {} | {} to {} | {} \n",
        proto,
        l3.get_source(),
        l4.get_source(),
        l3.get_destination(),
        l4.get_destination()
    );
    let payload = l4.get_payload();
    let len = payload.len();

    //show payload

    let WIDTH = 40;

    for i in 0..len{
        print!("{:02X}", payload[i]);

        if i % WIDTH == WIDTH -1 || i == len -1 {
            for _j in 0..WIDTH - 1 - (i % (WIDTH)){
                print!("   ");
            }

            print!("|  ");

            for j in i - i % WIDTH..=i {
                if payload[j].is_ascii_alphabetic(){
                    print!("{}", payload[j] as char);
                }
                else{
                    print!(".");
                }
            }
            println!();
        }
    }
    println!("{}", "=".repeat(WIDTH * 3));
    println!();
 }

pub trait GettableEndPoints {
    fn get_source(&self) -> String;
    fn get_destination(&self) -> String;
    fn get_payload(&self) -> &[u8];
}

impl<'a> GettableEndPoints for Ipv4Packet<'a> {
    fn get_source(&self)->String{
        self.get_source().to_string()
    }

    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }

    fn get_payload(&self)-> &[u8]{
        self.payload()
    }
}


impl<'a> GettableEndPoints for Ipv6Packet<'a> {
    fn get_source(&self)->String{
        self.get_source().to_string()
    }

    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }

    fn get_payload(&self)-> &[u8]{
        self.payload()
    }
}
impl<'a> GettableEndPoints for TcpPacket<'a> {
    fn get_source(&self)->String{
        self.get_source().to_string()
    }

    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }

    fn get_payload(&self)-> &[u8]{
        self.payload()
    }
}

impl<'a> GettableEndPoints for UdpPacket<'a> {
    fn get_source(&self)->String{
        self.get_source().to_string()
    }

    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }

    fn get_payload(&self)-> &[u8]{
        self.payload()
    }

}
