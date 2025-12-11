use std::{io::Write, net::{TcpListener, TcpStream}};

fn main() {
    let mut listener = TcpStream::connect("127.0.0.1:6969").unwrap();

    let start_header = "START";
    let end_header = "END";
    let message = "hello world";

    let buffer = [
        start_header.as_bytes(),
        message.as_bytes(),
        end_header.as_bytes()
    ].concat();

    listener.write(&buffer).unwrap();
    print!("Sender")
}