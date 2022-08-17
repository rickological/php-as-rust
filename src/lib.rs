pub mod filegetputcontents {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;

    pub fn test_file_get_put_contents() {
        // read
        let s: String = file_get_contents(&Path::new("/tmp/hello.txt"))
            .expect("Failed to read file");
        println!("Read data: {}", s.as_str());
        
        // write
        file_put_contents(&Path::new("/tmp/bye.txt"), &String::from("goodbye"))
            .expect("Failed to write file");
    }
    
    pub fn file_put_contents(filename: &Path, contents: &String) -> Result<usize, io::Error> { // returns bytes or Err
        let mut f = File::create(&filename)?; // create File and assign to or return io::Error
        f.write_all(contents.as_bytes())?; // write string to File or return io::Error
        Ok(contents.len()) // return Ok(number of bytes written)
    }
    
    pub fn file_get_contents(filename: &Path) -> Result<String, io::Error> { // returns String or an io::Error
        let mut f = File::open(filename)?; // returns the Err or assigns File to f
        let mut s = String::new(); // new String into s (mutable)
        f.read_to_string(&mut s)?; // returns the Err or assigns the value to s
        Ok(s) // returns OK(s) s is the String
    }
}

pub mod dirdirectory {
    use std::fs;
    use std::io;
    use std::path::Path;

    pub fn test_dir() {
        let mut files = dir(Path::new("/tmp/"))
            .expect("failed");
        
        for f in &mut files {
            println!("{:}", f);
        }
        
        println!("{:?}", files);
    }

    #[derive(Debug)]
    pub struct Directory {
        pub entries: Vec<String>,
        counter: usize,
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
            if self.counter < self.entries.len() {
                let item = &self.entries[self.counter];
                self.counter += 1;
                return Some(String::from(item))
            } else {
                return None;
            }
        }
    }

    pub fn dir(directory: &Path) -> Result<Directory, io::Error> {
        Directory::new(directory)
    }
}