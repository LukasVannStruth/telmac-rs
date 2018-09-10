use display::Display;
use keypad::Keypad;
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
            program_counter: 0,
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
        
    }
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

