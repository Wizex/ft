use std::io::{Write, Read, BufWriter, BufReader};
use std::net::TcpStream;
use std::env::args;
use std::error::Error;
use std::fs::File;
use ft::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args().skip(1);
    let addr = args.next().unwrap();
    let path = args.next().unwrap();

    let mut client = TcpStream::connect(addr)?;
    
    let file = File::open(path.clone())?;
    client.write_all(&path.len().to_be_bytes()).unwrap();
    client.write_all(path.as_bytes()).unwrap();

    let mut buf = std::io::BufReader::new(file);

    let mut buffer = [0u8; DEFAULT_BUF_SIZE];
    loop {
        match buf.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                println!("{n}");
                client.write_all(&buffer[..n])?
            }
            Err(e) => panic!("Error occured: {e}")
        }
    }

    Ok(())
}
