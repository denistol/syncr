use rsync::client::client::Client;

fn main() {
    if let Some(hd) = std::env::home_dir() {
        let _ = Client::new(&hd);
    }
}
