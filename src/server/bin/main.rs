// SERVER

use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;


fn send_file_to_stream(stream: &mut TcpStream) {
    let data = include_str!("C:\\Users\\denis\\syncr\\Cargo.toml");

    // let len = data.len();

    let header_bytes = "FILE_START".as_bytes();
    let data_bytes = data.as_bytes();
    let footer_bytes = "FILE_END".as_bytes();

    let mut res = Vec::new();

    res.extend_from_slice(header_bytes);
    res.extend_from_slice(data_bytes);
    // res.extend_from_slice(footer_bytes);
    
    stream.write(&res);
    stream.write(&footer_bytes);
    
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6969").unwrap();

    for x in listener.incoming() {
        if let Ok(mut stream) = x {
            println!("Has stream");
            send_file_to_stream(&mut stream);
            sleep(Duration::from_secs(2));
            send_file_to_stream(&mut stream);
        }
    };
}