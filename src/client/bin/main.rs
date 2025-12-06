// CLIENT

use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs}, sync::Arc,
};

const BUF_SIZE: usize = 1024;

enum Message {
    GetFiles(Vec<u8>),
    SendFiles(Vec<u8>),
    Invalid,
}

struct IncomingConnection {
    stream: TcpStream,
}
impl IncomingConnection {
    fn new(addr: impl ToSocketAddrs) -> Self {
        Self { stream: TcpStream::connect(addr).unwrap() }
    }

    fn create_file() -> File {
        File::create("C:\\Users\\denis\\syncr\\out.txt").unwrap()
    }

    fn read_message(&mut self) -> Message {
        let mut buf = [0u8; BUF_SIZE];
        let n = self.stream.read(&mut buf).unwrap_or(0);

        if n == 0 {
            return Message::Invalid;
        }

        let data = &buf[..n];

        if let Some(rest) = data.strip_prefix(b"GET_FILES") {
            return Message::GetFiles(rest.to_vec());
        }

        Message::Invalid
    }

    fn run (&mut self) {
        loop {
            self.read_message();
        }
    }
}

fn main() {
    let mut connection = IncomingConnection::new("127.0.0.1:6969");
    connection.run();

    loop {
        // let mut buf: [u8; 1024] = [0; 1024];
        // let n = con.read(&mut buf).unwrap_or(0);

        // let mut file_buffer: Vec<u8> = vec![];

        // if n != 0 {
        //     let content = &buf[0..n];
        //     let content_str = content;

        //     // FILE, SIZE, CONTENT

        //     if content_str.starts_with("FILE_START".as_bytes()) {
        //         let file_content = &content["FILE_START".as_bytes().len()..];
        //         file_buffer.extend_from_slice(file_content);
        //     }

        //     if content_str.ends_with("FILE_END".as_bytes()) {
        //         let file_content = &content[0.."FILE_END".as_bytes().len()];
        //         file_buffer.extend_from_slice(file_content);

        //         // File done
        //         let _ = f.write(&file_buffer);
        //     }
        // }
    }
}
