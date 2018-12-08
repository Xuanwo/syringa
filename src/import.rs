use std::fs;

use git2::Repository;
use walkdir::{DirEntry, WalkDir};


const AUR_URL: &str = "https://aur.archlinux.org";

pub fn import_aur(package: &str) {
    let url = format!("{}/{}.git", AUR_URL, package);
    println!("Clone {} into {} ...", url, package);

    let repo = match Repository::clone(&url, format!("{}", package)) {
        Ok(repo) => repo,
        Err(e) => {
            panic!("failed to clone: {}", e)
        }
    };

    for entry in WalkDir::new(package).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();
        if !path.trim_left_matches(package).starts_with("/.") {
            continue;
        }
        if entry.path().is_dir() {
            match fs::remove_dir_all(entry.path()) {
                Ok(_) => println!("{} removed", path),
                Err(e) => panic!("failed to remove: {}", e)
            };
        } else {
            match fs::remove_file(entry.path()) {
                Ok(_) => println!("{} removed", path),
                Err(e) => panic!("failed to remove: {}", e)
            };
        }
    }
}
