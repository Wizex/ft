use std::io::{Write, Read};
use std::net::TcpListener;
use std::env::args;
use std::error::Error;
use std::fs::File;
use ft::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args().skip(1);
    let addr = args.next().unwrap();

    let listener = TcpListener::bind(addr)?;
    let (mut socket, _) = listener.accept()?;

    let mut bytes = [0u8; 8];
    socket.read_exact(&mut bytes)?;
    let len = u64::from_be_bytes(bytes);
    
    let mut bytes = vec![0u8; len as usize];
    socket.read_exact(&mut bytes)?;
    let path = String::from_utf8(bytes)?;

    let file = File::create(path)?;
    let mut buf = std::io::BufWriter::new(file);

    let mut buffer = [0u8; DEFAULT_BUF_SIZE];
    loop {
        match socket.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                buf.write_all(&buffer[..n])?
            }
            Err(e) => panic!("Error occured: {e}")
        }
    }

    Ok(())
}

