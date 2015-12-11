extern crate bincode;
use bincode::rustc_serialize::encode;
use bincode::SizeLimit;

use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    let bytes = read_file("challenge.bin");
    
    let mut file = File::create("out").unwrap();
    for pair in bytes.iter() {
        file.write_all(&encode(pair, SizeLimit::Infinite).unwrap());
    }
}

fn read_file(file: &str) -> Vec<u16> {
    let mut file = File::open(file).unwrap();
    let mut bytes: Vec<u16> = Vec::new();

    let mut buffer = [0; 2];
    while file.read(&mut buffer).unwrap() > 0 {
        let value = ((buffer[0] as u16) << 8) | buffer[1] as u16;
        bytes.push(value);
    }

    return bytes;
}