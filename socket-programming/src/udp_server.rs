use std::io::{Read, Write};
use std::net::{UdpSocket};
use std::{str, thread};

// reference implementation
pub fn serve2(address: &str)->Result<(), failure::Error> {

    let server_socket = UdpSocket::bind(address)?;

    loop {
        let mut buffer = [ 0u8; 1024];

        let (size, src) = server_socket.recv_from(&mut buffer)?;

        debug! ("Handling data from {}", src);

        print!("{}", str::from_utf8(& buffer[..size])?);
        server_socket.send_to(&buffer, src)?;

    }
}  



pub fn serve(address: &str)->Result<(), failure::Error>{

    let listener = UdpSocket::bind(address)?;

    loop {

            let mut buffer = [ 0u8; 1024 ];
            let (nbytes, _) = listener.recv_from(& mut buffer)?;

            thread::spawn(move || {
                handler(buffer, nbytes).unwrap_or_else(| error | error!("{:?}", error))
            });
    }
}

fn handler(buffer: [u8;1024], nbytes: usize)->Result<(), failure::Error>{
 //debug!("Handling data from {}", stream.peer_addr()?);
    print!("{}", str::from_utf8(&buffer[..nbytes])?);
    //stream.write_all(&buffer[..nbytes])?;

    return Ok(());
}

