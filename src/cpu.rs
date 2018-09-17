use display::Display;
use keypad::Keypad;
//use std::fs::File;

//use std::collections::HashMap;

pub struct Cpu {
    pub opcode: u16,
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub index_register: u16,
    pub program_counter: u16,
    pub gfx: [bool; 2048],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub sp: u16,
    pub keypad: Keypad,
    pub display: Display,
}

fn read_opcode(memory: [u8; 4096], index: u16) -> u16 {
    u16::from(memory[index as usize]) << 8
        | u16::from(memory[(index + 1) as usize])
}
impl Cpu {
    /// Creates a new empty instance of the CPU Struct
    /// 
    /// TODO: fill the rest of this in as things change
    /// ## Arguments
    /// * 'args' - a hashmap that contains the parameters that are assigned to the new cpu.
    /// 
    /// Valid args keys: 
    /// * 'keypad'
    #[allow(clippy::let_and_return)]
    pub fn new(/*args: HashMap*/) -> Cpu {
        let new_cpu = Cpu {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            gfx: [false; 2048],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keypad: Keypad {},
            display: Display {}
        };
        
        new_cpu
    }
    
    
    pub fn execute_cycle(&mut self) {
        self.opcode = read_opcode(self.memory, self.program_counter);
        self.process_opcode();
    }   
    //TODO: determine if opcode needs to be parameter or can just be used as member.
    fn process_opcode(&mut self/*, opcode: u16*/) {
        match self.get2opbytes(0xF000) {
            0x0000 => self.opcode_0xxx(),
            0x1000 => self.opcode_1xxx(),
            0x2000 => self.opcode_2xxx(),
            0x3000 => self.opcode_3xxx(),
            0x4000 => self.opcode_4xxx(),
            0x5000 => self.opcode_5xxx(),
            0x6000 => self.opcode_6xxx(),
            0x7000 => self.opcode_7xxx(),
            0x8000 => self.opcode_8xxx(),
            0x9000 => self.opcode_9xxx(),
            0xA000 => self.opcode_Axxx(),
            0xB000 => self.opcode_Bxxx(),
            0xC000 => self.opcode_Cxxx(),
            0xD000 => self.opcode_Dxxx(),
            0xE000 => self.opcode_Exxx(),
            0xF000 => self.opcode_Fxxx(),
            _      => self.unimplemented_opcode_exception()
        }
    }

    
    fn opcode_0xxx(&mut self) {}

    /// ## Opcode 1xxx - JP addr
    /// 
    /// Jumps to the memory address specified in the latter 12 bits of the opcode.
    fn opcode_1xxx(&mut self) {
        self.program_counter = self.get2opbytes(0x0FFF);
    }
    
    /// ## Opcode 2xxx - CALL addr
    /// 
    /// Calls the subroutine at the address nnn
    fn opcode_2xxx(&mut self) {
        self.stack[self.sp as usize] = self.program_counter;
        self.sp += 1;
        self.program_counter = self.get2opbytes(0x0FFF);
    }   
    /// ## Opcode 3xkk - SE Vx, 
    /// 
    /// Compares the register Vx and kk and if they're equal, it skips the next instruction.
    fn opcode_3xxx(&mut self) {
        if self.v[(self.get2opbytes(0x0F00)) as usize] == self.getopbyte(0xFF) {
            self.program_counter += 2;
        } else {
            self.program_counter += 1;
        }
    }

    /// ## Opcode 4xkk - SNE Vx, kk
    /// 
    /// Compares the register Vx and kk and if they're not equal, it skips the next instruction.
    fn opcode_4xxx(&mut self) {
        if self.v[(self.get2opbytes(0x0F00)) as usize] != self.getopbyte(0xFF) {
            self.program_counter += 2;
        } else {
            self.program_counter += 1;
        }
    }

    /// ## Opcode 5xy0 - SE Vx, Vy
    /// 
    /// Compares the registers Vx and Vy, and if they're equal it skips the next instruction.
    fn opcode_5xxx(&mut self) {
        if self.v[(self.get2opbytes(0x0F00)) as usize] == self.v[(self.getopbyte(0xF0)) as usize] {
            self.program_counter += 2;
        } else {
            self.program_counter += 1;
        }
    }

    /// ## Opcode 6xkk - LD Vx, kk
    /// 
    /// Sets the value of register Vx to kk.
    fn opcode_6xxx(&mut self) {
        //NOTE: Because you can't borrow self mutably and immutably at the same time, getxopbyte
        //doesn't work here because it borrows immutably.
        let register_opcode = self.opcode;
        self.write_to_register(register_opcode & 0x0F00, (register_opcode & 0x00FF) as u8);
        self.program_counter += 1;
    }

    /// ## Opcode 7xkk - ADD Vx, kk
    /// 
    /// Adds the value kk to the register Vx
    fn opcode_7xxx(&mut self) {
        self.v[(self.get2opbytes(0x0F00)) as usize] += self.getopbyte(0xFF);
        self.program_counter += 1;
    }

    /// ## List of 8xxx Opcodes:
    /// 
    /// * 8xy0 - LD Vx, Vy - Stores the value of register Vy in Vx.
    /// * 8xy1 - OR Vx, Vy - Performs a bitwise OR on Vx and Vy, and stores the result in Vx.
    /// * 8xy2 - AND Vx, Vy - Performs a bitwise AND on Vx and Vy, and stores the result in Vx.
    /// * 8xy3 - XOR Vx, Vy - Performs a bitwise XOR on Vx and Vy, and stores the result in Vx.
    /// * 8xy4 - ADD Vx, Vy - Vx and Vy are added together. If the result is > 255(max value of a 
    /// byte), Vf is set to 1 and otherwise 0. Only the lowest 8 bits of the result are kept and 
    /// they are stored in Vx.
    /// * 8xy5 - SUB Vx, Vy - If Vx > Vy, Vf is set to 1 otherwise 0. Then Vy is subtracted from Vx 
    /// and the result is stored in Vx
    /// * 8xy6 - SHR Vx - If the least significant bit of Vx is 1, then Vf is set to 1, otherwise 0.
    /// Vx is then divided by 2.
    /// * 8xy7 - SUBN Vx, Vy - If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted
    /// from Vy, and the results stored in Vx.
    /// * 8xyE - SHL Vx - If the most-signiflcant bit of Vx is 1, then VF is set to 1, otherwise to 
    /// 0. Then Vx is multiplied by 2.
    fn opcode_8xxx(&mut self) {
        // #region register variable initalization.
        // TODO: move these into get functions for portability?
        let x_register: u8 = (self.opcode >> 8) as u8;
        let u16x_register: u16 = x_register.into();
        let y_register: u8 = (self.opcode & 0x00F0) as u8;
        let vx: u8 = self.get_register(x_register);
        let vy: u8 = self.get_register(y_register);
        // #endregion
        match self.getopbyte(0x0F) {
            //TODO: Strip newlines?
            
            0x0000 => { self.write_to_register(u16x_register, vy     ); },
            0x0001 => { self.write_to_register(u16x_register, vx | vy); },
            0x0002 => { self.write_to_register(u16x_register, vx & vy); },
            0x0003 => { self.write_to_register(u16x_register, vx ^ vy); },
            0x0004 => {
                // NOTE: this is a u16 because it needs to not overflow if the result is greater 
                // than 255.
                let add_result: u16 = (vx + vy).into();

                if add_result > u8::max_value().into() {
                    self.v[0xF as usize] = 1;
                } else {
                    self.v[0xF as usize] = 0;
                }
                self.write_to_register(u16x_register, (add_result & 0x00FF) as u8);
            },
            0x0005 => {
                if vx > vy {
                    self.v[0xF as usize] = 1;
                } else {
                    self.v[0xf as usize] = 0;
                }
                self.write_to_register(u16x_register, vx - vy);
            },
            0x0006 => {
                if vx & 0b1 == 1 {
                    self.write_to_register(0xF, 0b1);
                } else {
                    self.write_to_register(0xF, 0b0);
                }
                self.write_to_register(u16x_register, vx / 2);
            },
            0x0007 => {
                if vy > vx {
                    self.write_to_register(0xF, 1);
                    self.write_to_register(u16x_register, vy - vx);
                }
            },
            0x000E => {
                if vx >> 7 == 1 {
                    self.write_to_register(0xF, 0b1);
                } else {
                    self.write_to_register(0xF, 0b0);
                }
                self.write_to_register(u16x_register, vx * 2);
            },
            _     => self.unimplemented_opcode_exception()
        }
    }

    /// ## Opcode 9xy0 - SNE Vx, Vy - Skips next instruction if Vx != Vy.
    fn opcode_9xxx(&mut self) {
        let x_register: u8 = (self.opcode >> 8) as u8;
        let y_register: u8 = (self.opcode & 0x00F0) as u8;
        // FIXME: change this when refactoring x_register into functions.
        if self.get_register((self.get2opbytes(0x0F00) >> 8) as u8) != 
           self.get_register(self.getopbyte(0xF0)) {
               self.program_counter += 2;
           }
    }

    // NOTE: i like opcode_A stylistically, more than opcode_a so that is why these warnings are
    // disabled.
    #[allow(non_snake_case)]
    fn opcode_Axxx(&mut self) {}
    #[allow(non_snake_case)]
    fn opcode_Bxxx(&mut self) {}
    #[allow(non_snake_case)]
    fn opcode_Cxxx(&mut self) {}
    #[allow(non_snake_case)]
    fn opcode_Dxxx(&mut self) {}
    #[allow(non_snake_case)]
    fn opcode_Exxx(&mut self) {}
    #[allow(non_snake_case)]
    fn opcode_Fxxx(&mut self) {}
    
    //TODO: Refactor all instances of registers changing to use this function. 
    /// ## Writes a numeric value to a register. 
    /// 
    /// ### Arguments: 
    /// 
    /// * register: The hex digit in the opcode that indicates the register to change. 
    /// * value: The value to change the register to. 
    fn write_to_register(&mut self, register: u16, value: u8) {
        self.v[(self.opcode & register) as usize] = value;
    }
    //DECIDE: do i just make everything take a mut self so register opcode doesn't have to exist?
    fn get2opbytes(&self, bits: u16) -> u16 {
        self.opcode & bits
    }
    //DECIDE: Error handling?
    fn getopbyte(&self, bits: u8) -> u8 {
        self.opcode as u8 & bits
    }

    /* NOTE: this takes a mut so that there don't have to be copies created of self created for 
    every opcode that needs it. */
    fn get_register(&mut self, register: u8) -> u8 {
        self.v[register as usize]
    }
    fn unimplemented_opcode_exception(&self) {
        println!("Error, opcode: {} not implemented", self.opcode);
        
        //TODO: fail gracefully 
    }
}

#[cfg(test)]
mod tests {
    use cpu;
    use test::Bencher;

    #[test]
    //TODO: create tests for each opcode
    fn test_opcode_read() {
        let mut test_cpu = cpu::Cpu::new();
        test_cpu.memory[0] = 0b1111_0000;
        test_cpu.memory[1] = 0b0000_1111;
        test_cpu.opcode = cpu::read_opcode(test_cpu.memory, 0);
        assert_eq!(test_cpu.opcode, 0b1111_0000_0000_1111);
    }

    #[test]
    fn test_get2opbytes() {
        let mut test_cpu = cpu::Cpu::new();
        test_cpu.opcode = 0xF923;
        let result = test_cpu.get2opbytes(0x00FF);
        assert_eq!(0x0023, result);
    }

    #[test]
    fn test_getopbyte() {
        let mut test_cpu = cpu::Cpu::new();
        test_cpu.opcode = 0xF923;
        let result = test_cpu.getopbyte(0x00FF);
        assert_eq!(0x0023, result);
    }

    // Benchmark Tests
    /* NOTE: These are here because I wanted to see what would be faster out of curiosity but they
    both run in under a nanosecond.
    */
    #[bench]
    fn bench_get2opbytes(b: &mut Bencher) {
        b.iter(|| test_get2opbytes())
    }
    #[bench]
    fn bench_getopbyte(b: &mut Bencher) {
        b.iter(|| test_getopbyte())
    }

}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new()
    }
}