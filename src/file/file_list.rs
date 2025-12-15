use crate::constants::LOG_FILENAME;
use crate::file::file_item::FileItem;
use bincode;
use chrono::{DateTime, Utc};
use ignore::WalkBuilder;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::env::temp_dir;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn get_log_path() -> PathBuf {
    temp_dir().join(LOG_FILENAME)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileList {
    last_mod: Option<i64>,
    list: Vec<FileItem>,
    root_dir: PathBuf,
    ignored_dirs: Vec<PathBuf>,
}

impl FileList {
    pub fn new(root_dir: PathBuf) -> Self {
        println!("{:?}", get_log_path());
        let start = Instant::now();

        let mut list = FileList {
            last_mod: None,
            list: vec![],
            root_dir,
            ignored_dirs: vec![],
        };

        for i in ["Library", "Applications"] {
            list.ignored_dirs.push(list.root_dir.join(i));
        }

        // list.load_files();
        list.preload_from_cache();

        let duration = start.elapsed();
        // list.show_log();
        println!("Scan time: {:?} / Count: {}", duration, list.list.len());
        list
    }
    pub fn save_to_file(&self) {
        let file = File::create(get_log_path()).unwrap();
        let mut writer: BufWriter<File> = BufWriter::new(file);
        bincode::encode_into_std_write(&self.list, &mut writer, bincode::config::standard())
            .unwrap();
    }
    pub fn show_log(&self) {
        for x in &self.list {
            println!("{}", x);
        }
    }
    pub fn preload_from_cache(&mut self) {
        let file = File::open(get_log_path()).unwrap();
        let mut reader = BufReader::new(file);
        let list: Vec<FileItem> =
            bincode::decode_from_std_read(&mut reader, bincode::config::standard()).unwrap();
        self.list = list;
    }

    pub fn load_files(&mut self) {
        self.last_mod = None;
        self.list.clear();

        let ignored_dirs = self.ignored_dirs.clone();

        for entry in WalkBuilder::new(&self.root_dir)
            .filter_entry(move |entry| !ignored_dirs.iter().any(|d| entry.path().starts_with(d)))
            .hidden(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .build()
        {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                continue;
            }

            let meta = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let modified: DateTime<Utc> = match meta.modified() {
                Ok(m) => m.into(),
                Err(_) => continue,
            };

            self.list.push(FileItem::new(
                &entry.path(),
                meta.len(),
                Some(modified.timestamp()),
            ));

            self.last_mod = Some(
                self.last_mod
                    .map(|lm| lm.max(modified.timestamp()))
                    .unwrap_or(modified.timestamp()),
            );
        }
        self.list.sort();

        self.list.par_iter_mut().for_each(|item| {
            item.get_chunks();
        });

        self.show_log();
        self.save_to_file();
    }
}
