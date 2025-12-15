use std::{env, path::PathBuf};

use rsync::client::client::Client;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let p = PathBuf::from(env::var("ROOT_DIR").unwrap());
    let _ = Client::new(&p);
}
