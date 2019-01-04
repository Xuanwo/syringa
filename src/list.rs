use std::fs;

use atty::Stream;
use common::*;
use rayon::prelude::*;
use serde_yaml;
use walkdir::{DirEntry, WalkDir};

pub fn list(search: &str, is_maintainer: bool) {
    if is_maintainer {
        list_packages_by_maintainer(search)
    } else {
        unreachable!()
    }
}

fn list_packages_by_name() {}

fn list_packages_by_maintainer(search: &str) {
    if atty::is(Stream::Stdout) {
        println!("Packages maintained by {}", search);
        println!("--------");
    }

    for entry in WalkDir::new(".").max_depth(2) {
        let entry = entry.unwrap();
        if entry.file_name() != "lilac.yaml" {
            continue;
        }

        let mut f = fs::File::open(entry.path()).unwrap();

        let d: LilacYAML = serde_yaml::from_reader(&f).unwrap();

        let mut matched = false;

        if search.contains("@") {
            for m in &d.maintainers {
                if m.email == search {
                    matched = true;
                    continue;
                }
            }
        } else {
            for m in &d.maintainers {
                if m.name == search || m.github == search {
                    matched = true;
                    continue;
                }
            }
        }

        if !matched {
            continue;
        }

        let name: Vec<&str> = entry.path().to_str().unwrap().split("/").collect();

        println!("{}", name[1])
    }
}
