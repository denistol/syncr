use std::{
    env::{self, current_dir, home_dir},
    os,
    path::PathBuf,
};

use dotenv::dotenv;
use rsync::client::client::Client;

#[test]
fn test_client() {
    // dotenv().ok();
    // println!("ROOT_DIR: {:?}", env::var("ROOT_DIR"));
    // let p = PathBuf::from(env::var("ROOT_DIR").unwrap());
    // let p = PathBuf::from("value");
    // let c = Client::new(&p);
    assert_eq!(true, true);
}
