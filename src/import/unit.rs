use std::{
    collections::{hash_map, HashMap},
    fs,
    path::PathBuf,
};

#[derive(Debug)]
pub struct FileImport {
    entries: HashMap<String, FileEntry>,
}

impl FileImport {
    pub fn new() -> Self {
        Self {
            entries: hash_map::HashMap::new(),
        }
    }
    pub fn add_entry(&mut self, k: String, v: FileEntry) {
        self.entries.insert(k, v);
    }
}

#[derive(Debug)]
pub enum FileEntry {
    Single(String),
    Block(Block),
    Empty,
}
#[derive(Debug)]
pub enum BlockEntry {
    Str(String),
}
#[derive(Debug)]
pub struct Block {
    entries: Vec<BlockEntry>,
}

impl Block {
    pub fn new(entries: Vec<BlockEntry>) -> Self {
        Self { entries }
    }
}
pub fn handle_path(path: &str) -> i32 {
    let mut read = 0;
    match fs::metadata(path) {
        Ok(r) => {
            if r.is_file() {
                match fs::read_dir(PathBuf::from(path).parent().unwrap()) {
                    Ok(mut rd) => {
                        while let Some(Ok(f)) = rd.next() {
                            if f.path().to_str().unwrap() == path {
                                parse_file(&f, &mut read);
                            }
                        }
                        read
                    }
                    Err(_) => todo!(),
                }
            } else if r.is_dir() {
                read_dir(path)
            } else {
                read
            }
        }
        Err(_) => todo!(),
    }
}
fn read_dir(folder: &str) -> i32 {
    let mut read = 0;
    match fs::read_dir(folder) {
        Ok(mut rd) => {
            while let Some(Ok(f)) = rd.next() {
                // println!("{:?}", f.file_name());
                if f.metadata().unwrap().is_file() {
                    // read += 1;
                    parse_file(&f, &mut read);
                } else if f.metadata().unwrap().is_dir() {
                    read += read_dir(f.path().to_str().unwrap());
                }
            }
        }
        Err(_) => todo!(),
    }
    read
}

fn parse_file(f: &fs::DirEntry, read: &mut i32) {
    let file_str = fs::read_to_string(f.path()).unwrap();
    if f.file_name().to_str().unwrap().ends_with(".mtf") {
        // println!("{}", file_str);
        if let Some(_) = parse_mtf_file(file_str) {
            *read += 1
        }
    } else if f.file_name().to_str().unwrap().ends_with(".blk") {
        // println!("{:?}", f.file_name());
        if let Some(_) = parse_blk_file(file_str) {
            *read += 1
        }
    } else {
        println!("new extension: {:?}", f.file_name());
    }
}

fn parse_mtf_file(file_str: String) -> Option<FileImport> {
    let mut file = FileImport::new();
    let mut lines = file_str.split("\r\n");
    while let Some(line) = lines.next() {
        if line.is_empty() || line.starts_with("#") {
            continue;
        } else if line.ends_with(":") || line.starts_with("Weapons:") {
            let header = line;
            let mut entries = vec![];
            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }
                entries.push(BlockEntry::Str(line.to_string()));
            }

            let b = Block::new(entries);
            file.add_entry(header.to_string(), FileEntry::Block(b));
            // println!("{:?}", b);
        } else {
            if let Some(idx) = line.find(":") {
                file.add_entry(
                    line[0..idx].to_string(),
                    FileEntry::Single(line[idx + 1..].to_string()),
                )
            }
        }
    }
    println!("{:#?}", file);
    Some(file)
}

fn parse_blk_file(file_str: String) -> Option<FileImport> {
    let mut file = FileImport::new();
    let mut lines = file_str.split("\r\n");
    while let Some(line) = lines.next() {
        if line.is_empty() || line.trim().starts_with("#") {
            continue;
        }
        if let Some((tag_name, end_tag)) = parse_start_end_tags(line) {
            let mut entries = vec![];
            while let Some(untrimmed_val) = lines.next() {
                let val = untrimmed_val.trim();
                if val == end_tag {
                    match entries.len() {
                        0 => {
                            // println!("Empty record for {}", tag_name);
                            file.add_entry(tag_name, FileEntry::Empty);
                        }
                        1 => {
                            // println!("Parsed Block {}: {}", tag_name, val);
                            if let Some(BlockEntry::Str(e)) = entries.pop() {
                                file.add_entry(tag_name, FileEntry::Single(e))
                            }
                        }
                        _ => {
                            let b = Block::new(entries);
                            // println!("{:?}", b);
                            file.add_entry(tag_name, FileEntry::Block(b));
                        }
                    }
                    break;
                } else if val.is_empty() && tag_name != "notes" {
                    println!(
                        "Badly formed BLK file. Expected {} found empty line",
                        end_tag
                    );
                    return None;
                } else if val.starts_with("#") || val.is_empty() {
                    continue;
                } else {
                    entries.push(BlockEntry::Str(val.to_string()));
                }
            }
        } else {
            println!("Badly formed BLK file.");
            return None;
        }
    }
    Some(file)
}

fn parse_start_end_tags(tag: &str) -> Option<(String, String)> {
    let mut end_tag = tag.to_string();
    if let Some(string) = tag.strip_suffix(">") {
        if let Some(string) = string.strip_prefix('<') {
            end_tag.insert(1, '/');
            Some((string.to_string(), end_tag.clone().to_string()))
        } else {
            println!("None in Prefix");
            None
        }
    } else {
        println!("None in Suffix");
        None
    }
}
