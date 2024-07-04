
// CONSTANTS
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;

const STACK_SIZE: usize = 16;

const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


pub struct Emu {
  pc: u16,
  // RAM array
  ram: [u8; RAM_SIZE],
  // Screen represented by boolean array
  screen: [bool; SCREEN_SIZE],
  // V register to improve performance because RAM access is slow
  v_reg: [u8; NUM_REGS],
  // I register for indexing RAM 
  i_reg: u16,
  // Stack pointer
  sp: u16,
  // Stack array to enter and exit subroutine
  stack: [u16; STACK_SIZE],
  // Boolean array for which of 16 keys is being pressed
  keys: [bool; NUM_KEYS],
  // Delay Timer
  dt: u8,
  // Sound Timer 
  st: u8,
}

impl Emu {
  // Emu intialization
  pub fn new() -> Self {
    let mut new_emu = Self {
      pc: START_ADDR,
      ram: [0; RAM_SIZE],
      screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
      v_reg: [0; NUM_REGS],
      i_reg: 0,
      sp: 0,
      stack: [0; STACK_SIZE],
      keys: [false; NUM_KEYS],
      dt: 0,
      st: 0,
  };

  new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

  new_emu
  }

  // Reset Emu object without creating a new one
  pub fn reset(&mut self) {
    self.pc = START_ADDR;
    self.ram = [0; RAM_SIZE];
    self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    self.v_reg = [0; NUM_REGS];
    self.i_reg = 0;
    self.sp = 0;
    self.stack = [0; STACK_SIZE];
    self.keys = [false; NUM_KEYS];
    self.dt = 0;
    self.st = 0;
    self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
}
  
  // Push to stack
  fn push(&mut self, val: u16) {
    self.stack[self.sp as usize] = val;
    self.sp += 1;
  }

  // Pop from stack
  fn pop(&mut self) -> u16 {
    self.sp -= 1;
    self.stack[self.sp as usize]
  }
}



#[cfg(test)]
mod tests {

}
