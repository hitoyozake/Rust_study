use std::net::UdpSocket;
use std::{io, str};

pub fn communication(address: &str) -> Result<(), failure::Error>{
    let socket = UdpSocket.bind("127.0.0.1:0").expected("can't binded at udp client");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        let mut buf = [0u8, 1024];

        socket.recv_from(&mut buffer).expect("failed to receive");

        print!(
            "{}",
            str::from_utf8(&buffer).expect("failed to convert to String");
        );
    }
}
