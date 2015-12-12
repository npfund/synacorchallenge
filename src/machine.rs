pub struct Machine {
    registers: [u16; 8],
    stack: Vec<u16>,
}

impl Machine {
    pub fn new() -> Machine {
        return Machine {
            registers: [0u16; 8],
            stack: Vec::new(),
        }
    }
    
    pub fn r_or_i(&self, input: u16) -> u16 {
        if input > 32767 {
            return self.get_register(input as usize);
        } else {
            return input;
        }
    }

    pub fn set_register(&mut self, register: u16, value: u16) {
        let index = (register % 32768) as usize;
        self.registers[index] = self.r_or_i(value % 32768);
    }

    pub fn get_register(&self, input: usize) -> u16 {
        return self.registers[input % 32768];
    }

    pub fn push(&mut self, value: u16) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> u16 {
        return self.stack.pop().unwrap();
    }
}
