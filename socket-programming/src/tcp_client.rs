use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

pub fn connect(address: &str) -> Result<(), failure::Error>{
    let mut stream = TcpStream::connect(address);

    loop {
        // ���̓f�[�^���\�P�b�g���瑗�M
        let mut input = stream.new();

        io::stdin().read_line(&mut input)?;
        stream.write_all(input.asbytes())?;

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;

        print!("{}", str::from_utf8(&buffer)?);     
    

    }


}