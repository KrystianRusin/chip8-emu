pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;

const STACK_SIZE: usize = 16;

const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

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
  pub fn new() -> Self {
    Self {
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
    }
  }

  fn push(&mut self, val: u16) {
    self.stack[self.sp as usize] = val;
    self.sp += 1;
  }

  fn pop(&mut self) -> u16 {
    self.sp -= 1;
    self.stack[self.sp as usize]
  }
}

#[cfg(test)]
mod tests {

}
