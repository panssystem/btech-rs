use std::{fs, ops::Index, path::PathBuf};

use btech_rs::import::handle_path;

fn main() {
    // let folder = "C:\\Users\\peter\\Repos\\Battletech\\megamek\\megamek\\data\\mechfiles";
    let folder = "C:\\Users\\peter\\Repos\\Battletech\\megamek\\megamek\\data\\mechfiles\\mechs\\3039u\\Zeus ZEU-6T.mtf";
    let total_read = handle_path(folder);
    println!("total files read: {}", total_read);
    // let val = fs::read_to_string(path)
}
