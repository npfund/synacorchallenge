use std::fs::File;
use std::io;
use std::io::Read;
use std::char;
use std::collections::VecDeque;

mod machine;
use machine::Machine;

fn main() {
    let debug = false;
    let mut machine = Machine::new();
    let mut bytes = read_file("src/challenge.bin");

    let mut input_buffer = String::new();
    let mut input_chars: VecDeque<char> = VecDeque::new();

    let mut index: usize = 0;
    while index < bytes.len() {
        let instruction = bytes[index];
        match instruction {
            0 => break,
            1 => {
                let value = machine.r_or_i(bytes[index + 2]);
                if debug {
                    println!("{}: Set register {} to {}", index, bytes[index + 1], value);
                }
                machine.set_register(bytes[index + 1], value);
                index += 3;
            }
            2 => {
                let value = machine.r_or_i(bytes[index + 1]);
                if debug {
                    println!("{}: Push {}", index, bytes[index + 1]);
                }
                machine.push(value);
                index += 2;
            }
            3 => {
                let value = machine.pop();
                if debug {
                    println!("{}: Pop {} into {}", index, value, bytes[index + 1]);
                }
                machine.set_register(bytes[index + 1], value);
                index += 2;
            }
            4 => {
                if debug {
                    println!("{}: Check {} == {}",
                             index,
                             bytes[index + 2],
                             bytes[index + 3]);
                }
                if machine.r_or_i(bytes[index + 2]) == machine.r_or_i(bytes[index + 3]) {
                    if debug {
                        println!("{}: Set register {} to {}", index, bytes[index + 1], 1);
                    }
                    machine.set_register(bytes[index + 1], 1);
                } else {
                    if debug {
                        println!("{}: Set register {} to {}", index, bytes[index + 1], 0);
                    }
                    machine.set_register(bytes[index + 1], 0);
                }
                index += 4;
            }
            5 => {
                if debug {
                    println!("{}: Check {} > {}",
                             index,
                             bytes[index + 2],
                             bytes[index + 3]);
                }
                if machine.r_or_i(bytes[index + 2]) > machine.r_or_i(bytes[index + 3]) {
                    if debug {
                        println!("{}: Set register {} to {}", index, bytes[index + 1], 1);
                    }
                    machine.set_register(bytes[index + 1], 1);
                } else {
                    if debug {
                        println!("{}: Set register {} to {}", index, bytes[index + 1], 0);
                    }
                    machine.set_register(bytes[index + 1], 0);
                }
                index += 4;
            }
            6 => {
                if debug {
                    println!("{}: Jump to {}", index, bytes[index + 1]);
                }
                index = bytes[index + 1] as usize;
            }
            7 => {
                if debug {
                    println!("{}: Check {} != 0", index, bytes[index + 1]);
                }
                if machine.r_or_i(bytes[index + 1]) != 0 {
                    if debug {
                        println!("{}: Jump to {}", index, bytes[index + 2]);
                    }
                    index = bytes[index + 2] as usize;
                } else {
                    if debug {
                        println!("{}: No jump", index);
                    }
                    index += 3;
                }
            }
            8 => {
                if debug {
                    println!("{}: Check {} == 0", index, bytes[index + 1]);
                }
                if machine.r_or_i(bytes[index + 1]) == 0 {
                    if debug {
                        println!("{}: Jump to {}", index, bytes[index + 2]);
                    }
                    index = bytes[index + 2] as usize;
                } else {
                    if debug {
                        println!("{}: No jump", index);
                    }
                    index += 3;
                }
            }
            9 => {
                let value = machine.r_or_i(bytes[index + 2]) + machine.r_or_i(bytes[index + 3]);
                if debug {
                    println!("{}: Set register {} to {}", index, bytes[index + 1], value);
                }
                machine.set_register(bytes[index + 1], value);
                index += 4;
            }
            10 => {
                let value =
                    (machine.r_or_i(bytes[index + 2]) as u32 *
                     machine.r_or_i(bytes[index + 3]) as u32) as u16;
                if debug {
                    println!("{}: MULT Set register {} to {}",
                             index,
                             bytes[index + 1],
                             value);
                }
                machine.set_register(bytes[index + 1], value);
                index += 4;
            }
            11 => {
                let value = machine.r_or_i(bytes[index + 2]) % machine.r_or_i(bytes[index + 3]);
                if debug {
                    println!("{}: MOD Set register {} to {}",
                             index,
                             bytes[index + 1],
                             value);
                }
                machine.set_register(bytes[index + 1], value);
                index += 4;
            }
            12 => {
                let value = machine.r_or_i(bytes[index + 2]) & machine.r_or_i(bytes[index + 3]);
                if debug {
                    println!("{}: AND Set register {} to {}",
                             index,
                             bytes[index + 1],
                             value);
                }
                machine.set_register(bytes[index + 1], value);
                index += 4;
            }
            13 => {
                let value = machine.r_or_i(bytes[index + 2]) | machine.r_or_i(bytes[index + 3]);
                if debug {
                    println!("{}: OR Set register {} to {}",
                             index,
                             bytes[index + 1],
                             value);
                }
                machine.set_register(bytes[index + 1], value);
                index += 4;
            }
            14 => {
                let value = !machine.r_or_i(bytes[index + 2]);
                if debug {
                    println!("{}: NOT Set register {} to {}",
                             index,
                             bytes[index + 1],
                             value);
                }
                machine.set_register(bytes[index + 1], value);
                index += 3;
            }
            15 => {
                let address = machine.r_or_i(bytes[index + 2]) as usize;
                if debug {
                    println!("{}: Set register {} to {}",
                             index,
                             bytes[index + 1],
                             bytes[address]);
                }
                machine.set_register(bytes[index + 1], bytes[address]);
                index += 3;
            }
            16 => {
                let address = machine.r_or_i(bytes[index + 1]) as usize;
                if debug {
                    println!("{}: Set register {} to {}",
                             index,
                             bytes[address],
                             machine.r_or_i(bytes[index + 2]));
                }
                bytes[address] = machine.r_or_i(bytes[index + 2]);
                index += 3;
            }
            17 => {
                if debug {
                    println!("{}: Push {}; jump to {}",
                             index,
                             index + 2,
                             machine.r_or_i(bytes[index + 1]));
                }
                machine.push((index + 2) as u16);
                index = machine.r_or_i(bytes[index + 1]) as usize;
            }
            18 => {
                let value = machine.pop() as usize;
                if debug {
                    println!("{}: Return to {}", index, value);
                }
                index = value;
            }
            19 => {
                let value = char::from_u32(machine.r_or_i(bytes[index + 1]) as u32).unwrap();
                if debug {
                    println!("{}: Print {}", index, value);
                }
                print!("{}", value);
                index += 2;
            }
            20 => {
                let value;

                match input_chars.pop_front() {
                    Some(c) => value = c as u16,
                    None => {
                        match io::stdin().read_line(&mut input_buffer) {
                            Ok(_) => {
                                for c in input_buffer.chars() {
                                    input_chars.push_back(c);
                                }
                                input_buffer = "".to_string();
                                value = input_chars.pop_front().unwrap() as u16;
                            }
                            Err(error) => {
                                println!("{}: {}", index, error);
                                continue;
                            }
                        }
                    }
                }

                if debug {
                    println!("{}: Read {} into {}", index, value, bytes[index + 1]);
                }
                machine.set_register(bytes[index + 1], value);
                index += 2;
            }
            21 => {
                if debug {
                    println!("{}: NOOP", index);
                }
                index += 1;
            }
            _ => {
                println!("{}: Unknown instruction {}", index, instruction);
                index += 1
            }
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
