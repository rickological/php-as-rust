use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // read
    let s: String = file_get_contents(&Path::new("/tmp/hello.txt"))
        .expect("Failed to read file");
    println!("Read data: {}", s.as_str());
    
    // write
    file_put_contents(&Path::new("/tmp/bye.txt"), &String::from("goodbye"))
        .expect("Failed to write file");
}

fn file_put_contents(filename: &Path, contents: &String) -> Result<usize, io::Error> { // returns bytes or Err
    let mut f = File::create(&filename)?; // create File and assign to or return io::Error
    f.write_all(contents.as_bytes())?; // write string to File or return io::Error
    Ok(contents.len()) // return Ok(number of bytes written)
}

fn file_get_contents(filename: &Path) -> Result<String, io::Error> { // returns String or an io::Error
    let mut f = File::open(filename)?; // returns the Err or assigns File to f
    let mut s = String::new(); // new String into s (mutable)
    f.read_to_string(&mut s)?; // returns the Err or assigns the value to s
    Ok(s) // returns OK(s) s is the String
}