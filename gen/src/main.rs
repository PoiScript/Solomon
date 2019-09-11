#![feature(proc_macro_hygiene)]

mod entry;
mod error;
mod handlers;
mod json;
mod rss;

use std::{fs, path::Path};

use crate::entry::Entry;
use crate::error::Result;
use crate::json::{write_json, write_posts_json};

fn walk_dirs<P: AsRef<Path>>(dir: P, files: &mut Vec<String>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk_dirs(&path, files)?;
        } else if let Some(ext) = path.extension() {
            if ext == "org" {
                files.push(fs::read_to_string(&path)?);
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut files = Vec::new();
    walk_dirs("content/post", &mut files)?;

    let mut entries = files
        .iter()
        .map(|file| Entry::from(file))
        .collect::<Result<Vec<_>>>()?;

    entries.sort_by(|a, b| b.date.cmp(&a.date));

    fs::create_dir_all("assets/post")?;

    write_json(&entries)?;
    write_posts_json(&entries)?;

    let about_org = fs::read_to_string("content/about.org")?;
    let about_entry = Entry::from(&about_org)?;

    write_json(&[about_entry])?;

    fs::write("assets/atom.xml", rss::markup(&entries).into_string())?;

    Ok(())
}
