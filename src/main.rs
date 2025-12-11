mod client;
mod message;
use std::{env::args, thread::sleep, time::Duration};

use crate::client::Client;

#[allow(unused)]
#[allow(dead_code)]

fn main() {
    let arguments: Vec<String> = args().collect();

    let base_path = "C:\\Users\\denis\\rsync\\testdir";

    let mut client = Client::new(base_path);

    client.show_info();
    client.run();

}
