use std::{env::home_dir, os};

use rsync::client::client::Client;

#[test]
fn test_client() {
    let p = home_dir().unwrap();
    let c = Client::new(&p);
    assert_eq!(true, true);
}
