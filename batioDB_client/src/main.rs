#![feature(receiver_trait)]

use std::io::{Read, Stdin, stdin};
use tokio::sync::mpsc::{Sender,Receiver};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use crate::client_config::ClientConfig;

mod client_config;

// Batiodb client program
#[tokio::main]
async fn main() {
    let config = ClientConfig::new("config.yaml");
    let mut client = TcpStream::connect(config.port.clone()).await.unwrap();
    println!("connection: {:#?}",client);
    println!("baby turn it on!!!");
    let (mut tx,mut rx):(Sender<String>,Receiver<String>) = tokio::sync::mpsc::channel(32);

    tokio::spawn(async move {
        let (mut reader,mut writer) = tokio::io::split(client);
        let mut recv: Vec<u8> = vec![0u8; 1024];

        loop {
            tokio::select! {
                sender = rx.recv() => {
                    let mut message = sender.unwrap();
                    let message = message.replace("\n", "").into_bytes();
                    writer.write_all(message.as_slice()).await.unwrap();
                }
                res = reader.read(&mut recv) => {
                    let value_size = res.unwrap();
                    if value_size == 0 {
                        break;
                    }
                    let value = std::str::from_utf8(&recv.as_slice()[..value_size]).unwrap();
                    println!("value: {:?}",value);
                }
            }
        }

    });

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("reading from stdin failed");
        tx.send(buffer.clone()).await.unwrap();
        buffer.clear();
    }
}
