use std::{fs};
use std::path::PathBuf;

pub struct TreeEngine {
    // given path to tree on
    path: String,

    // use ascii or unicode to chart
    is_ascii: bool, 

    // show file name or not
    display_name: bool,

    // max recursive depth in search
    max_depth: u32,

    // sort file in directory
    // current available methods: time, name, size, last modified
    sort: String,

    // should sort be increamental or decreamental
    sort_direction: bool,

    // ignored file types
    ignore: Vec<String>,
}

pub fn tree(prefix: String, path: PathBuf) -> anyhow::Result<()> {
    if prefix.is_empty() {
        println!("{}", path.to_string_lossy());
    }
    let first_entries = fs::read_dir(&path)?;
    let mut counts: i32 = 0;
    for entry in first_entries {
        if entry.is_ok() {
            counts += 1;
        }
    }
    let entries = fs::read_dir(&path)?;
    for entry in entries {
        let entry = entry?;
        let mut sub_prefix = " │  ";
        if counts != 1 {
            println!("{} ├─ {}", prefix, entry.file_name().to_string_lossy());
        } else {
            println!("{} └─ {}", prefix, entry.file_name().to_string_lossy());
            sub_prefix = "    ";
        }
        if entry.file_type()?.is_dir() {
            let next_prefix: String = (prefix.clone() + sub_prefix).to_string();
            let mut next_path = path.clone();
            next_path.push(entry.file_name());
            tree(next_prefix, next_path)?;
        }
        counts -= 1;
    }
    Ok(())
}