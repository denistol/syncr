use std::{io::Write, net::TcpStream};

use rsync::constants::{END_HEADER, START_HEADER};


fn main() {
    let mut listener = TcpStream::connect("127.0.0.1:6969").unwrap();


    let mut msg: Vec<u8> = vec![];
    msg.reserve(300);
    

    let buffer = [
        START_HEADER,
        &msg,
        END_HEADER,
    ]
    .concat();

    listener.write(&buffer).unwrap();
    print!("Sender")
}
