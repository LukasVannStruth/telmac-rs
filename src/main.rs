extern crate telmacrs;
//use telmacrs::cpu::Cpu;

fn main() {
/* notes from 
// Set up render system and register input callbacks
  setupGraphics();
  setupInput();
 
  // Initialize the Chip8 system and load the game into the memory  
  myChip8.initialize();
  myChip8.loadGame("pong");
 
  // Emulation loop
  for(;;)
  {
    // Emulate one cycle
    myChip8.emulateCycle();
 
    // If the draw flag is set, update the screen
    if(myChip8.drawFlag)
      drawGraphics();
 
    // Store key press state (Press and Release)
    myChip8.setKeys();	
  }

    Line 3-5: In this example we assume you will create a separate class to handle the opcodes.
    Line 10-11: Setup the graphics (window size, display mode, etc) and input system (bind callbacks)
    Line 14: Clear the memory, registers and screen
    Line 15: Copy the program into the memory
    Line 21: Emulate one cycle of the system
    Line 24: Because the system does not draw every cycle, we should set a draw flag when we need to update our screen. Only two opcodes should set this flag:
        0x00E0 – Clears the screen
        0xDXYN – Draws a sprite on the screen
    Line 28: If we press or release a key, we should store this state in the part that emulates the keypad

*/


}
