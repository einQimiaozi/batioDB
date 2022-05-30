use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::{Sender,Receiver};
use crate::{DB, Parse};
use crate::parse::Command;

pub struct Message {
    resp: oneshot::Sender<Vec<u8>>,
    buffer: Vec<u8>,
}

impl Message {
    pub fn new(resp: oneshot::Sender<Vec<u8>>,buffer: Vec<u8>) -> Self {
        Message {
            resp: resp,
            buffer: buffer,
        }
    }
}

pub struct DBService {}

impl DBService {
    pub async fn listen(mut db: DB<'static>, address: &str, channel_cap: usize) {
        let (mut tx, mut rx):(Sender<Message>, Receiver<Message>) = mpsc::channel(channel_cap);
        let listener = TcpListener::bind(address).await.unwrap();

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                let command = Parse::parse(message.buffer);
                let mut value = String::new();
                match command.command {
                    Command::GET => {
                        println!("get");
                        value = match db.get(command.key) {
                            Some(T) => T,
                            None => "nil".to_string(),
                        };
                    },
                    Command::SET | Command::UPDATE => {
                        println!("set");
                        db.put(command.key, command.value);
                        value = "success".to_string();
                    },
                    Command::DELETE => {
                        println!("delete");
                        db.remove(command.key);
                        value = "success".to_string();
                    },
                    Command::None => {
                        println!("command error");
                        value = "command invalid".to_string();
                    },
                }
                println!("{:?}", value);
                message.resp.send(value.into_bytes());
            }
        });

        while let Ok((socket, address)) = listener.accept().await {
            let tx = tx.clone();
            tokio::spawn(async move {
                println!("client info: {}", address);
                // 开始接收消息
                DBService::process(socket, tx).await;
            });
        }
    }

    async fn process(socket: TcpStream, tx: Sender<Message>) {
        let mut buffer = [0; 1024];
        let (mut reader,mut writer) = tokio::io::split(socket);
        loop {
            let buffer_size = reader.read(&mut buffer).await.unwrap();
            if buffer_size == 0 {
                break;
            }
            let (signal_tx,signal_rx):(oneshot::Sender<Vec<u8>>,oneshot::Receiver<Vec<u8>>) = oneshot::channel();
            let buffer = buffer[0..buffer_size].to_vec();
            let message = Message::new(signal_tx,buffer);
            tx.send(message).await;

            let res = signal_rx.await.unwrap();
            writer.write_all(res.as_slice()).await.unwrap();
        }
    }
}