use clap::Parser;
use ft::*;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter, Write};
use std::net::TcpStream;

pub type Result = std::result::Result<(), Box<dyn Error>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// File transfer type
    #[arg(value_enum)]
    transfer_type: TransferType,

    /// Remote server address
    host: String,

    /// Local path
    local_path: String,

    /// Remote path
    remote_path: String,
}

/// Connects to a host and does file transferring.
pub fn start() -> Result {
    let cli = Cli::parse();

    let mut socket = TcpStream::connect(cli.host)?;

    // Writes a transfer type
    let bytes = bincode::serialize(&cli.transfer_type)
        .expect("Abnormal error. Can't serialize transfer type.");

    socket.write_all(&(bytes.len() as u32).to_be_bytes())?;
    socket.write_all(&bytes)?;

    // Writes a path
    socket.write_all(&(cli.remote_path.len() as u32).to_be_bytes())?;
    socket.write_all(cli.remote_path.as_bytes())?;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(cli.local_path)?;

    match cli.transfer_type {
        TransferType::To => io::copy(&mut BufReader::new(file), &mut BufWriter::new(socket))?,
        TransferType::From => io::copy(&mut BufReader::new(socket), &mut BufWriter::new(file))?,
    };

    Ok(())
}
