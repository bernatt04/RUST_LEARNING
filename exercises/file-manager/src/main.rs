use std::{
    fs::{self, File},
    io::{Read, Write},
};

fn main() {
    let mut file = File::create("test.txt").unwrap();
    file.write_all(b"hello world").unwrap(); //b convets text to bytes

    let content = fs::read_to_string("test.txt").unwrap();
    print!("{}", content);
}
