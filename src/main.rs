use std::{
    collections::HashMap,
    io::Read,
    net::{IpAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex}, thread,
};
use std::io::Write;
mod client;
mod message;

use chrono::Utc;
use client::Client;
use message::Message;

fn handle_stream(mut stream: TcpStream, clients: Arc<Mutex<HashMap<IpAddr, Client>>>) {
    let addr = stream.peer_addr().unwrap().ip();
    {
        let mut clients = clients.lock().unwrap();
        if !clients.contains_key(&addr) {
            println!("Client connected: {}", addr);
            clients.insert(addr, Client::new(stream.try_clone().unwrap()));
        }
    }

    let mut buffer = [0; 24];

    let remove_client = || {
        println!("Client disconnected: {}", &addr);
        clients.lock().unwrap().remove(&addr);
    };

    match stream.read(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                remove_client();
            } else if n == 1 {
                match Message::from_byte(buffer[0]) {
                    Some(Message::GetDir) => {
                        println!("GetDir command");
                    },
                    Some(Message::ShowDir) => {
                        println!("ShowDir command");
                    },
                    _ => {
                        println!("Invalid command {}", buffer[0]);
                    }
                }
            }
            else {
                println!("Received {} bytes from {}", n, addr);
                println!("Received bytes: {:?}", &buffer[0..n]);
                let mut clients = clients.lock().unwrap();
                if let Some(client) = clients.get_mut(&addr) {
                    client.last_message_at = Some(Utc::now());
                };
            }
            write!(stream, "PONG").expect("ERROR: pong");
        }
        _ => { remove_client() }
    }
}


fn main() {
    let clients: Arc<Mutex<HashMap<IpAddr, Client>>> = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("0.0.0.0:6969").expect("ERROR: TcpListener bind");

    for l in listener.incoming() {
        if let Ok(stream) = l {
            let clients = clients.clone();
            thread::spawn(move|| {
                handle_stream(stream, clients);
            });
        };
    }
}
