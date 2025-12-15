use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    path::PathBuf,
    sync::{
        Arc, Mutex,
        mpsc::{self, SyncSender},
    },
    thread::{self},
};

use crate::{
    BUFFER_SIZE, END_HEADER, START_HEADER, file::file_list::FileList, sync_event::SyncEvent,
};

pub struct Client {
    pub file_list: FileList,
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
    pub fn new(root_dir: &PathBuf) -> Self {
        if !root_dir.exists() {
            panic!("Path {:?} not exists!", &root_dir);
        }
        let file_list = FileList::new(root_dir.to_path_buf());
        Self { file_list }
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

        for r in rx.iter() {
            // println!("{}", r.to_string())
        }
        t2.join().unwrap();
    }
}
