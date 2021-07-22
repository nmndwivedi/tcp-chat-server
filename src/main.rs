use std::net::SocketAddr;

use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{TcpListener}, sync::broadcast};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(16);
    
    loop {
        let (mut sock, addr) = listener.accept().await.unwrap();

        let (tx, mut rx) = (tx.clone(), tx.subscribe());

        tokio::spawn(async move {
            let (read, mut write) = sock.split();

            let mut reader = BufReader::new(read);
            let mut line = String::new();

            loop {
                tokio::select! {
                    read_bytes = reader.read_line(&mut line) => {
                        if read_bytes.unwrap()==0 { break; }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    },
                    msg = rx.recv() => {
                        let msg = msg.unwrap();
                        if msg.1 == addr { continue; }
                        let out = addr.to_string() + ": " + &msg.0;
                        write.write_all(out.as_bytes()).await.unwrap();
                    }
                }
            }
        });
        
    }
}
