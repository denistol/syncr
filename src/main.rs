use std::{env, path::PathBuf};

use dotenv::dotenv;
use rsync::client::client::Client;

fn main() {
    dotenv().ok();
    let p = PathBuf::from(env::var("ROOT_DIR").unwrap());
    let _ = Client::new(&p);
}
