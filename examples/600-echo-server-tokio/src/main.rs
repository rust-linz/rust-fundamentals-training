use std::io::Result;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

async fn echo(mut socket: TcpStream) -> Result<()> {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);

    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).await?;
        if buffer.trim() == "quit" {
            break;
        }
        writer.write_all(buffer.as_bytes()).await?;
    }
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:8082").await?;
    loop {
        if let Ok((socket, addr)) = listener.accept().await {
            println!("Listening to {}", addr);
            tokio::task::spawn(echo(socket));
        }
    }
}
