use std::{
    collections::{hash_map, HashMap},
    fs,
    path::PathBuf,
};

use crate::{
    constants::imports::*,
    units::{BattleMech, MechConfig, UnitType},
};

#[cfg(feature = "vehicle")]
use crate::units::Vehicle;

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
pub struct UnitConversionError(&str);

impl TryInto<UnitType> for FileImport {
    type Error = UnitConversionError;
    
    fn try_into(self) -> Result<UnitType, Self::Error> {
        #[cfg(any(feature = "vehicle", feature = "aerospace", feature = "infantry"))]
        if let Some(FileEntry::Single(unit_type)) = self.entries.get(UNIT_TYPE) {
            match unit_type.as_str() {
                #[cfg(feature = "aerospace")]
                TANK_TYPE =>
                {
                    #[cfg(feature = "vehicle")]
                    Ok(UnitType::Vehicle(Vehicle {
                        name: file_entry_to_string(self.entries.get(UNIT_NAME).unwrap()),
                        vehicle_type: VehicleType::Combat,
                        movement_mode: match self.entries.get(MOTION_TYPE).unwrap() {
                            FileEntry::Single(e) => MoveType::Tracked,
                            FileEntry::Block(_block) => todo!(),
                            FileEntry::Empty => todo!(),
                        },
                    }))
                }
                _ => Err(UnitConversionError("Invalid non-mech")),
            };
            todo!("Parse all units types except meks")
        }
        if self.entries.contains_key(MECH_CONFIG) {
            let config = match self.entries.get(MECH_CONFIG) {
                Some(FileEntry::Single(cfg)) if cfg == "Biped" => MechConfig::Biped,
                Some(FileEntry::Single(cfg)) if cfg == "Biped Omnimech" => MechConfig::BipedOmni,
                Some(FileEntry::Single(cfg)) if cfg == "Biped" => MechConfig::Quad,
                Some(FileEntry::Single(cfg)) if cfg == "Biped Omnimech" => MechConfig::QuadOmni,
                Some(FileEntry::Single(cfg)) if cfg == "Biped" => MechConfig::QuadVee,
                Some(FileEntry::Single(cfg)) if cfg == "Biped Omnimech" => MechConfig::QuadVeeOmni,
                Some(_) => MechConfig::Unknown,
                None => MechConfig::Unknown,
            };
            Ok(UnitType::Mech(BattleMech {
                config,
                chassis: match self.entries.get(MECH_CHASSIS) {
                    Some(FileEntry::Single(chassis)) => chassis.to_string(),
                    _ => "Unknown".to_string(),
                },
                model: match self.entries.get(MECH_MODEL) {
                    Some(FileEntry::Single(model)) => model.to_string(),
                    _ => "Unknown".to_string(),
                },
                myomer: todo!(),
                structure: todo!(),
                locations: todo!(),
                mass: todo!(),
                armor: todo!(),
            }))
        } else {
            Err(UnitConversionError("Invalid mech"))
        }
    }
    
}

#[derive(Debug)]
pub enum FileEntry {
    Single(String),
    Block(Block),
    Empty,
}

fn file_entry_to_string(entry: &FileEntry) -> String {
    match entry {
        FileEntry::Single(s) => s.clone(),
        _ => String::new(),
    }
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
        Err(e) => todo!("{:#?}", e),
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
    let mut lines = file_str.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() || line.starts_with("#") {
            continue;
        } else if line.ends_with(":") || line.starts_with("Weapons:") {
            let split_header: Vec<&str> = line.split(":").collect();
            let header = split_header[0];
            // println!("{:#?}",split_header);
            let mut entries = vec![];
            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }
                entries.push(BlockEntry::Str(line.to_string()));
            }
            if split_header.len() == 2 && split_header[1] != "" {
                println!("{:#?}", split_header);
                assert_eq!(entries.len().to_string(), split_header[1], "Weapon Count doesn't match.");
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
    let mut lines = file_str.lines();
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
