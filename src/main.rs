mod entry;
use entry::get_entries;
use std::env::args;

#[allow(unused)]
#[allow(dead_code)]

fn main() {
    let arguments: Vec<String> = args().collect();

    let base_path = "C:\\Users\\denis\\rsync\\testdir";

    let entry_list = get_entries(base_path).unwrap();
    
    for i in entry_list.items {
        println!("{}", i);
    }
}
