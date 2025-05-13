use super::cpu::*;
use super::memory::*;
use super::screen::*;
use super::keys::*;

use std::time::{Instant, Duration};
use std::thread::sleep;

 
pub const INSTRUCTIONS_PER_SECOND: u128 = 500;
pub const TIMERS_DECREASE_RATE: u128 = 60;
pub const TIMERS_TICK: u128 = 1_000_000_000 / TIMERS_DECREASE_RATE;

pub struct Chip8{
    pub cpu: Cpu,
    pub screen: Screen,
    pub memory: Memory,
    pub keys: Keys
}

impl Chip8 {
    pub fn new(rom_file: &mut Vec<u8>) -> Chip8{
        Chip8{
            cpu: Cpu::new(),
            memory: Memory::new(rom_file),
            screen: Screen::new(),
            keys: Keys::new(),
        }
    }


 


}