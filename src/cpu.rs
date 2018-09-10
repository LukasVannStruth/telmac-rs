use display::Display;
use keypad::Keypad;
use std::fs::File;
use os;
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
/* memory notes
0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
0x200-0xFFF - Program ROM and work RAM
*/
fn read_word(memory: [u8; 4096], index: u16) -> u16 {
    (memory[index as usize] as u16) << 8
        | (memory[(index + 1) as usize] as u16)
}
impl Cpu {
    /// Creates a new empty instance of the CPU Struct
    /// 
    /// TODO: fill the rest of this in as things change
    /// # Arguments
    /// * 'args' - a hashmap that contains the parameters that are assigned to the new cpu.
    /// 
    /// Valid args keys: 
    /// * 'keypad'
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
        // this is for loading the fontset into memory
        // for address in (0..80) {
            // new_cpu.memory[address] = chip8_fontset[address];
        // }
        new_cpu
    }
    pub fn execute_cycle(&mut self) {
        let opcode: u16 = read_word(self.memory, self.program_counter);
        self.process_opcode(opcode);
    }   
    fn process_opcode(&mut self, opcode: u16) {
        match self.opcode & 0xF000 {
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
            0xF000 => self.opcode_Fxxx()
            //TODO: implement not implemented error.
        }
    }

    fn opcode_0xxx(&mut self) {}
    fn opcode_1xxx(&mut self) {}
    fn opcode_2xxx(&mut self) {}    
    fn opcode_3xxx(&mut self) {}
    fn opcode_4xxx(&mut self) {}
    fn opcode_5xxx(&mut self) {}
    fn opcode_6xxx(&mut self) {}
    fn opcode_7xxx(&mut self) {}
    fn opcode_8xxx(&mut self) {}
    fn opcode_9xxx(&mut self) {}
    fn opcode_Axxx(&mut self) {}
    fn opcode_Bxxx(&mut self) {}
    fn opcode_Cxxx(&mut self) {}
    fn opcode_Dxxx(&mut self) {}
    fn opcode_Exxx(&mut self) {}
    fn opcode_Fxxx(&mut self) {}

}

#[cfg(test)]
mod tests {
    use cpu;
    #[test]
    fn test_opcode_read() {
        let mut test_cpu = cpu::Cpu::new();
        test_cpu.memory[0] = 0b1111_0000;
        test_cpu.memory[1] = 0b0000_1111;
        test_cpu.opcode = cpu::read_word(test_cpu.memory, 0);
        assert_eq!(test_cpu.opcode, 0b1111_0000_0000_1111);
    }

}

