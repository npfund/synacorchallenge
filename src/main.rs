use std::fs::File;
use std::io::Read;
use std::char;

fn main() {
    let bytes = read_file("challenge.bin");
    
    let mut index = 0;
    while index < bytes.len() {
        let instruction = bytes[index];
        match instruction {
            0 => break,
            19 => {
                print!("{}", char::from_u32(bytes[index + 1] as u32).unwrap());
                index += 1;
            },
            21 => index += 1,
            _ => {
                //println!("Unknown instruction {}", instruction);
                index += 1
            },
        }
    }
}

fn read_file(file: &str) -> Vec<u16> {
    let mut file = File::open(file).unwrap();
    let mut bytes: Vec<u16> = Vec::new();

    let mut buffer = [0; 2];
    while file.read(&mut buffer).unwrap() > 0 {
        let value = ((buffer[1] as u16) << 8) | buffer[0] as u16;
        bytes.push(value);
    }

    return bytes;
}