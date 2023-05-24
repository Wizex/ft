use ft::*;
use std::env::args;
use std::error::Error;
use tokio::fs::OpenOptions;
use tokio::io::{self, AsyncReadExt, BufStream};
use tokio::net::{TcpListener, TcpStream};

pub type Result = std::result::Result<(), Box<dyn Error + Send + Sync + 'static>>;

/// Listens for new incoming connections and handles file transferring.
///
/// Handles an every connection as a separate asynchronous task.
pub async fn serve() -> Result {
    let mut args = args().skip(1);
    let addr = args
        .next()
        .ok_or("Expect an address to listen to for connections")?;

    let listener = TcpListener::bind(addr).await?;
    loop {
        let (socket, _) = match listener.accept().await {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Failed to accept an incoming connection. Reason: {err}");
                continue;
            }
        };
        tokio::spawn(handle_connection(socket));
    }
}

async fn handle_connection(mut socket: TcpStream) -> Result {
    // Reads a transfer type
    let transfer_type = {
        let len = socket.read_u32().await?;
        let mut payload = vec![0u8; len as usize];
        socket.read_exact(&mut payload).await?;
        bincode::deserialize::<TransferType>(&payload)?
    };

    // Reads a path
    let path = {
        let len = socket.read_u32().await?;
        let mut bytes = vec![0u8; len as usize];
        socket.read_exact(&mut bytes).await?;
        String::from_utf8(bytes)?
    };

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .await?;

    let mut socket = BufStream::new(socket);
    let mut file = BufStream::new(file);

    match transfer_type {
        TransferType::From => io::copy(&mut file, &mut socket).await?,
        TransferType::To => io::copy(&mut socket, &mut file).await?,
    };

    Ok(())
}
