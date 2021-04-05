use std::cell::Cell;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::{
    self,
    io::{AsyncRead, AsyncReadExt},
};
pub mod gemini;
use gemini::DefaultServer;
use gemini::Server;
mod util;
#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("localhost:1965").await?;
    while let Ok((mut socket, _addr)) = listener.accept().await {
        tokio::spawn(async move {
            let mut buffer: [u8; 1024] = [0; 1024];
            let server = gemini::DefaultServer;
            let request_vec = get_request_url(&mut socket, &mut buffer).await?;

            if let Ok(request_string) = String::from_utf8(request_vec) {
                let response = server.process(request_string.clone());
                socket.write_all(&response.to_bytes()).await
            } else {
                socket.write_all(ERROR.as_bytes()).await
            }
        });
    }
    Ok(())
}

async fn get_request_url(socket: &mut TcpStream, buffer: &mut [u8]) -> tokio::io::Result<Vec<u8>> {
    let mut request_url: Vec<u8> = Vec::with_capacity(1024);

    'read: loop {
        let last_character: Cell<Option<u8>> = Cell::from(None);
        match socket.read(buffer).await {
            Ok(n) if n == 0 => break,
            Ok(n) => {
                for byte in &buffer[..n] {
                    request_url.push(*byte);
                    if last_character.get() == Some(0x0D) && Some(*byte) == Some(0x0A) {
                        break 'read;
                    }
                    last_character.set(Some(*byte))
                }
            }
            Err(e) => return Err(e),
        };
    }
    Ok(request_url)
}
const REPLY: &str = "OK";
const ERROR: &str = "ERR";
