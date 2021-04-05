use tokio::net::TcpListener;
use tokio::{self};
pub mod gemini;

use gemini::Server;
use tokio::io::AsyncWriteExt;
mod util;
#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("localhost:1965").await?;
    while let Ok((mut socket, _addr)) = listener.accept().await {
        tokio::spawn(async move {
            let server = gemini::DefaultServer;
            let request_string = gemini::get_request_url(&mut socket).await?;

            let response = server.process(request_string.clone());
            socket.write_all(&response.to_bytes()).await
        });
    }
    Ok(())
}
