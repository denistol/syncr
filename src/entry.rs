use std::{
    fmt::Display,
    fs::{DirEntry, read_dir},
    path::{Path, PathBuf},
    time::SystemTime,
};

pub struct ItemEntry {
    pub dir_entry: DirEntry,
    pub relative_path: PathBuf,
    pub size: u64,
    pub last_mod: SystemTime,
}

impl Display for ItemEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_path = self.dir_entry.path();
        write!(
            f,
            "FULL_PATH: {}, RELATIVE_PATH: {}, LAST_MOD: {:?}, SIZE: {}",
            full_path.display(),
            self.relative_path.display(),
            self.last_mod,
            self.size
        )
    }
}

pub struct EntryList {
    pub items: Vec<ItemEntry>,
}

impl EntryList {
    pub fn new(items: Vec<ItemEntry>) -> Self {
        Self { items: items }
    }
}

fn get_files(base_path: &Path) -> Vec<DirEntry> {
    let mut files = Vec::new();

    if let Ok(entries) = read_dir(base_path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                files.extend(get_files(&path));
            } else if path.is_file() {
                files.push(entry);
            }
        }
    }

    files
}

pub fn get_entries(base_path: &str) -> Result<EntryList, std::io::Error> {
    let base_path = Path::new(base_path);
    let entries = get_files(base_path);

    let mut items = Vec::new();

    for entry in entries {
        let path = entry.path();
        if let Ok(metadata) = entry.metadata() {
            if let Ok(relative_path) = path.strip_prefix(base_path) {
                if let Ok(last_mod) = metadata.modified() {
                    items.push(ItemEntry {
                        dir_entry: entry,
                        relative_path: relative_path.to_path_buf(),
                        size: metadata.len(),
                        last_mod,
                    });
                }
            }
        }
    }

    Ok(EntryList::new(items))
}
