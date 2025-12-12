use rsync::Client;


#[test]
fn test_client() {
    let c = Client::new(".");
    c.show_info();
    assert_ne!(c.current_files.len(), 0);
}