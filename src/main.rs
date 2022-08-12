use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let files = dir(Path::new("/tmp/"))
        .expect("failed");
    
    for f in files {
        println!("{:}", f);
    }
    // println!("{:?}", files);
}

#[derive(Debug)]
pub struct Directory {
    pub entries: Vec<String>,
    pub counter: usize,
}

impl Directory {
    pub fn new(directory: &Path) -> Result<Self, io::Error> {
        let mut paths: Vec<String> = Vec::new();
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            paths.push(path.as_path().display().to_string());
        }
        Ok(Self {
            entries: paths,
            counter: 0,
        })
    }
}

impl Iterator for Directory {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        if self.counter < self.entries.len() {
            let item = &self.entries[self.counter];
            return Some(String::from(item))
        } else {
            return None;
        }
    }
}

fn dir(directory: &Path) -> Result<Directory, io::Error> {
    Directory::new(directory)
}