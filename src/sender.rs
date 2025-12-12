use std::{io::Write, net::TcpStream};

use rsync::constants::{END_HEADER, START_HEADER};


fn main() {
    let mut listener = TcpStream::connect("127.0.0.1:6969").unwrap();

    let message = include_str!("C:\\Users\\denis\\rsync\\Cargo.toml");

    let buffer = [
        START_HEADER,
        message.as_bytes(),
        END_HEADER,
    ]
    .concat();

    listener.write(&buffer).unwrap();
    print!("Sender")
}
