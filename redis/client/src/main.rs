use commands::ClientCmd;
use std::io;
use tokio::net::UdpSocket;

mod client;

static SERVER_IP: &str = "127.0.0.1";
static SERVER_PORT: &str = "8000";

#[tokio::main]
async fn main() -> io::Result<()> {
    client::welcome();

    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let server_addr = format!("{}:{}", SERVER_IP, SERVER_PORT);
    sock.connect(server_addr).await?;

    loop {
        match client::get_client_cmd() {
            ClientCmd::Get(key) => {
                let data = serde_json::to_string(&ClientCmd::Get(key))?;
                sock.send(data.as_bytes()).await?;
            }
            ClientCmd::Set(key, value) => {
                let data = serde_json::to_string(&ClientCmd::Set(key, value))?;
                sock.send(data.as_bytes()).await?;
            }
            ClientCmd::Exit => break,
        };

        let mut buf = [0; 1024];
        let len = sock.recv(&mut buf).await?;

        let data = &buf[..len];
        let response = String::from_utf8(data.to_vec()).expect("Invalid UTF-8");
        println!("{}", response);
    }

    Ok(())
}
