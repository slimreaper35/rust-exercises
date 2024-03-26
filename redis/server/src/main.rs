use commands::ClientCmd;
use std::collections::HashMap;
use std::io;
use tokio::net::UdpSocket;

static SERVER_IP: &str = "127.0.0.1";
static SERVER_PORT: &str = "8000";

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = format!("{}:{}", SERVER_IP, SERVER_PORT);
    let sock = UdpSocket::bind(addr).await?;

    let mut map: HashMap<String, String> = HashMap::new();

    let mut buf = [0; 1024];

    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let data: ClientCmd = serde_json::from_slice(&buf[..len])?;
        println!("Received {:?} from {}", data, addr);

        match data {
            ClientCmd::Get(key) => {
                let value = map.get(&key);
                match value {
                    Some(value) => sock.send_to(value.as_bytes(), addr).await?,
                    None => sock.send_to(b"Not found", addr).await?,
                };
            }
            ClientCmd::Set(key, value) => {
                map.insert(key, value);
                sock.send_to(b"OK", addr).await?;
            }
            ClientCmd::Exit => continue,
        }
    }
}
