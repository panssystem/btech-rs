use btech_rs::import::handle_path;
use std::{env, path::PathBuf};

fn main() {
    let default_path = "C:\\Users\\peter\\Repos\\Battletech\\megamek\\megamek\\data\\mechfiles\\mechs\\3039u\\Zeus ZEU-6T.mtf".to_string();
    println!("Current Directory: {:#?}", env::current_dir());
    let args: Vec<_> = env::args().collect();
    if let Ok(mut folder) = env::current_dir() {
        folder.push(if args.len() > 1 {
            &args[1]
        } else {
            &default_path
        });
        // let folder = "C:\\Users\\peter\\Repos\\Battletech\\megamek\\megamek\\data\\mechfiles";
        // let folder = "C:\\Users\\peter\\Repos\\Battletech\\megamek\\megamek\\data\\mechfiles\\mechs\\3039u\\Zeus ZEU-6T.mtf";
        println!("Trying to read '{:#?}'", folder);
        let total_read = handle_path(folder.to_str().unwrap());
        println!("total files read: {}", total_read);
        // let val = fs::read_to_string(path)
    }
}
