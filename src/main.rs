use std::fs::File;
use std::io::Read;
use std::char;

mod machine;
use machine::Machine;

fn main() {
    let mut machine = Machine::new();
    let bytes = read_file("challenge.bin");
    
    let mut index: usize = 0;
    while index < bytes.len() {
        let instruction = bytes[index];
        match instruction {
            0 => break,
            1 => {
                machine.set_register(bytes[index + 1], bytes[index + 2]);
                index += 3;
            },
            2 => {
                let value = machine.r_or_i(bytes[index + 1]);
                machine.push(value);
                index += 2;
            },
            3 => {
                let value = machine.pop();
                machine.set_register(bytes[index + 1], value);
                index += 2;
            },
            4 => {
                if machine.r_or_i(bytes[index + 2]) == machine.r_or_i(bytes[index + 3]) {
                    machine.set_register(bytes[index + 1], 1);
                } else {
                    machine.set_register(bytes[index + 1], 0);
                }
                index += 4;
            },
            5 => {
                if machine.r_or_i(bytes[index + 2]) > machine.r_or_i(bytes[index + 3]) {
                    machine.set_register(bytes[index + 1], 1);
                } else {
                    machine.set_register(bytes[index + 1], 0);
                }
                index += 4;
            }
            6 => index = bytes[index + 1] as usize,
            7 => if machine.r_or_i(bytes[index + 1]) != 0 {
                index = bytes[index + 2] as usize;
            } else {
                index += 3;
            },
            8 => if machine.r_or_i(bytes[index + 1]) == 0 {
                index = bytes[index + 2] as usize;
            } else {
                index += 3;
            },
            9 => {
                let value = machine.r_or_i(bytes[index + 2]) + machine.r_or_i(bytes[index + 3]);
                machine.set_register(bytes[index + 1], value);
                index += 4;
            },
            12 => {
                let value = machine.r_or_i(bytes[index + 2]) & machine.r_or_i(bytes[index + 3]);
                machine.set_register(bytes[index + 1], value);
                index += 4;
            },
            13 => {
                let value = machine.r_or_i(bytes[index + 2]) | machine.r_or_i(bytes[index + 3]);
                machine.set_register(bytes[index + 1], value);
                index += 4;
            },
            14 => {
                let value = !machine.r_or_i(bytes[index + 2]);
                machine.set_register(bytes[index + 1], value);
                index += 3;
            }
            19 => {
                print!("{}", char::from_u32(bytes[index + 1] as u32).unwrap());
                index += 2;
            },
            21 => index += 1,
            _ => {
                println!("Unknown instruction {}", instruction);
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

