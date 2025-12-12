#![allow(dead_code)]
#![allow(unused)]

use std::{env::args, thread::sleep, time::Duration};
use rsync::client::Client;

fn main() {
    let arguments: Vec<String> = args().collect();

    let base_path = "C:\\Users\\denis\\rsync\\testdir";

    let mut client = Client::new(base_path);

    client.show_info();
    client.run();
}
