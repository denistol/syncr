use std::{
    fs::{DirEntry, read_dir},
    io::Read,
    net::{TcpListener, TcpStream},
    path::PathBuf,
    sync::{
        Arc, Mutex,
        mpsc::{self, SyncSender},
    },
    thread::{self},
    time::SystemTime,
};

use crate::{BUFFER_SIZE, END_HEADER, START_HEADER, sync_event::SyncEvent};

pub struct Client {
    pub base_path: PathBuf,
    pub current_files: Vec<DirEntry>,
    pub last_mod: Option<SystemTime>,
}




fn handle_stream(stream: Arc<Mutex<TcpStream>>, sender: &SyncSender<SyncEvent>) {
    
    let mut buffer = [0; BUFFER_SIZE];
    let mut event = SyncEvent::new();

    loop {
        {
            let mut s = stream.lock().unwrap();
            let bytes = s.read(&mut buffer).unwrap_or(0);

            if bytes == 0 {
                break;
            };

            let message = event.parse(&buffer);
        };

    }
}

impl Client {
    pub fn show_info(&self) {
        println!("LAST MOD: {:?}", self.last_mod);
        for x in &self.current_files {
            println!("{:?}", x);
        }
    }

    pub fn new(base_path: &str) -> Self {
        let p = PathBuf::from(base_path);
        if !p.exists() {
            panic!("Path {:?} not exists!", &p);
        }
        let mut client = Client {
            base_path: PathBuf::from(base_path),
            current_files: vec![],
            last_mod: None,
        };
        client.load_files(&PathBuf::from(base_path));
        client
    }

    pub fn load_files(&mut self, base_path: &PathBuf) {
        if let Ok(entries) = read_dir(base_path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    self.load_files(&path);
                } else if path.is_file() {
                    if let Ok(last_mod) = path.metadata().and_then(|m| m.modified()) {
                        self.last_mod =
                            Some(self.last_mod.map_or(last_mod, |prev| prev.max(last_mod)));
                    }
                    self.current_files.push(entry);
                }
            }
        }
    }

    pub fn run(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
        let (tx, rx) = mpsc::sync_channel::<SyncEvent>(16);

        let t2 = thread::spawn(move || {
            for z in listener.incoming() {
                let sender = tx.clone();
                if let Ok(stream) = z {
                    let stream = Arc::new(Mutex::new(stream));
                    thread::spawn(move || {
                        handle_stream(stream.clone(), &sender);
                    });
                }
            }
        });

        println!("[*] Receiver loop started ...");

        for r in rx.iter() {
            println!("{}", r.to_string())
        }
        t2.join().unwrap();
    }
}
