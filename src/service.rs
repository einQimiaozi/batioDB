use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::{Sender,Receiver};
use crate::{DB, Parse};
use crate::parse::Command;

/*
    The server uses the reactor model and uses one to many channels to deliver messages.
    The message contains data and a one-to-one oneshot channel.
    Allocate a spawn for each client socket to handle read requests.
    The read request is read by the process method and encapsulated into a message, which is sent through the channel.
    An independent spawn continuously obtains messages from the channel, parses each message into a command,
    sends it to the database for execution, and sends it out through the oneshot channel in the message to notify the process that the request has been processed.
    process method writes back the processed request to the client.
 */

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

    // Start a DB service and start listening to ports
    pub async fn listen(mut db: DB<'static>, address: &str, channel_cap: usize) {
        let (mut tx, mut rx):(Sender<Message>, Receiver<Message>) = mpsc::channel(channel_cap);
        let listener = TcpListener::bind(address).await.unwrap();

        // If there are any messages in the channel, process them through this spawn.
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

        // All requests will be sent to the channel, so the service is concurrent and secure.
        while let Ok((socket, address)) = listener.accept().await {
            let tx = tx.clone();
            tokio::spawn(async move {
                println!("client info: {}", address);
                DBService::process(socket, tx).await;
            });
        }
    }

    // The read request is encapsulated as a message.
    // The process method will create a oneshot channel for each request for event driven and notification writeback.
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