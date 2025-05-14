use super::memory::*;
use crate::chip8::Chip8;
use std::iter::Inspect;
extern crate rand;

use self::rand::thread_rng;
use self::rand::Rng;

pub struct Cpu {
    pub registers: [u8; 16],
    pub pc: u16,
    pub i: usize,
    pub delay: u8,
    pub sound: u8,
    pub sp: usize,
    pub stack: [u16; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            pc: 0x200,
            i: 0,
            delay: 0,
            sound: 0,
            sp: 0,
            stack: [0; 16],
        }
    }
    pub fn handle_timers(&mut self) {
        if self.delay != 0 {
            self.delay -= 1;
        }
        if self.sound != 0 {
            self.sound -= 1;
        }
    }

    fn run(c8: &mut Chip8) {}

    pub fn execute(c8: &mut Chip8, ins: Instruction) {
        if ins.0 == 0x00E0 {
            cls(c8);
            return;
        }
        if ins.0 == 0x00EE {
            ret(c8);
            return;
        }
        let op = ins.op();
        let nnn = ins.nnn();
        let nibble = ins.nibble();
        let x = ins.x();
        let y = ins.y();
        let byte = ins.byte();
        match op {
            0x0 => sys(c8, nnn),
            0x1 => jp_addr(c8, nnn),
            0x2 => call(c8, nnn),
            0x3 => se_vx_byte(c8, x, byte),
            0x4 => sne_vx_byte(c8, x, byte),
            0x5 => se_vx_vy(c8, x, y),
            0x6 => ld_vx_byte(c8, x, byte),
            0x7 => add_vx_byte(c8, x, byte),
            0x8 => match ins.nibble() {
                //Instructions starting with 0x8 are Register Operations
                0x0 => ld_vx_vy(c8, x, y),
                0x1 => or_vx_vy(c8, x, y),
                0x2 => and_vx_vy(c8, x, y),
                0x3 => xor_vx_vy(c8, x, y),
                0x4 => add_vx_vy(c8, x, y),
                0x5 => sub_vx_vy(c8, x, y),
                0x6 => shr_vx(c8, x, y),
                0x7 => subn_vx_vy(c8, x, y),
                0xE => shl_vx(c8, x, y),
                _ => panic!("Unknown Instruction!"), //TODO better handling of unknown instruction
            },
            0x9 => sne_vx_vy(c8, x, y),
            0xA => ld_addr(c8, nnn),
            0xB => jp_addr(c8, nnn),
            0xC => rnd_vx_byte(c8, x, byte),
            0xD => draw_vx_vy_nibble(c8, x, y, nibble),
            0xE => match ins.byte() {
                0x9E => skp_vx(c8, x),
                0xA1 => sknp_vx(c8, x),
                _ => panic!("Unknown Instruction!"), //TODO better handling of unknown instruction
            },
            0xF => match ins.byte() {
                0x07 => ld_vx_dt(c8, x),
                0x0A => ld_vx_key(c8, x),
                0x15 => ld_dt_vx(c8, x),
                0x18 => ld_st_vx(c8, x),
                0x1E => add_i_vx(c8, x),
                0x29 => ld_f_vx(c8, x),
                0x33 => ld_bcd_vx(c8, x),
                0x55 => ld_i_vx(c8, x),
                0x65 => ld_vx_i(c8, x),
                _ => panic!("Unknown Instruction!"), //TODO better handling of unknown instruction
            },

            _ => panic!("Unknown Instruction!"), //TODO better handling of unknown instruction
        }
    }
}

pub fn fetch(c8: &Chip8, pc: u16) -> Instruction {
    let ins = c8.memory.read_word(pc as usize);
    print!("{:04x}  ", pc);
    print!("{:04x}  ", ins);
    println!();
    Instruction(ins)
}

pub fn step(c8: &mut Chip8) {
    let ins = fetch(c8, c8.cpu.pc);
    println!("Executing opcode: {:04x}", ins.0);
    Cpu::execute(c8, ins);
}

fn cls(c8: &mut Chip8) {
    c8.screen.data = [[false; 64]; 32];
    c8.cpu.pc += 2;
}

fn ret(c8: &mut Chip8) {
    c8.cpu.sp -= 1;
    c8.cpu.pc = c8.cpu.stack[c8.cpu.sp];
}

fn sys(c8: &mut Chip8, nnn: usize) {
    c8.cpu.pc = nnn as u16;
}

fn jp_addr(c8: &mut Chip8, nnn: usize) {
    c8.cpu.pc = nnn as u16;
    println!("Got jp_addr");
}

fn call(c8: &mut Chip8, nnn: usize) {
    c8.cpu.stack[c8.cpu.sp] = c8.cpu.pc + 2;
    c8.cpu.sp += 1;
    c8.cpu.pc = nnn as u16;
}

fn se_vx_byte(c8: &mut Chip8, x: usize, byte: u8) {
    if c8.cpu.registers[x] == byte {
        c8.cpu.pc += 2;
    }
    c8.cpu.pc += 2;
}

fn sne_vx_byte(c8: &mut Chip8, x: usize, byte: u8) {
    if c8.cpu.registers[x] != byte {
        c8.cpu.pc += 2;
    }
    c8.cpu.pc += 2;
}
fn se_vx_vy(c8: &mut Chip8, x: usize, y: usize) {
    if c8.cpu.registers[x] == c8.cpu.registers[y] {
        c8.cpu.pc += 2;
    }
    c8.cpu.pc += 2;
}

fn ld_vx_byte(c8: &mut Chip8, x: usize, byte: u8) {
    c8.cpu.registers[x] = byte;
    c8.cpu.pc += 2;
    println!("Got ld_vx_byte");
}

fn add_vx_byte(c8: &mut Chip8, x: usize, byte: u8) {
    let result = c8.cpu.registers[x].wrapping_add(byte);
    c8.cpu.registers[x] = result;
    c8.cpu.pc += 2;
}

fn ld_vx_vy(c8: &mut Chip8, x: usize, y: usize) {
    c8.cpu.registers[x] = c8.cpu.registers[y];
    c8.cpu.pc += 2;
}

fn or_vx_vy(c8: &mut Chip8, x: usize, y: usize) {
    c8.cpu.registers[x] |= c8.cpu.registers[y];
    c8.cpu.pc += 2;
}

fn and_vx_vy(c8: &mut Chip8, x: usize, y: usize) {
    c8.cpu.registers[x] &= c8.cpu.registers[y];
    c8.cpu.pc += 2;
}

fn xor_vx_vy(c8: &mut Chip8, x: usize, y: usize) {
    c8.cpu.registers[x] ^= c8.cpu.registers[y];
    c8.cpu.pc += 2;
}

fn add_vx_vy(c8: &mut Chip8, x: usize, y: usize) {
    let (result, carry) = c8.cpu.registers[x].overflowing_add(c8.cpu.registers[y]);
    c8.cpu.registers[x] = result;
    if carry {
        c8.cpu.registers[0xF] = 1;
    } else {
        c8.cpu.registers[0xF] = 0;
    }

    c8.cpu.pc += 2;
}

fn sub_vx_vy(c8: &mut Chip8, x: usize, y: usize) {
    c8.cpu.registers[0xF] = if c8.cpu.registers[x] > c8.cpu.registers[y] {
        1
    } else {
        0
    };
    let result = c8.cpu.registers[x].wrapping_sub(c8.cpu.registers[y]);
    c8.cpu.registers[x] = result;
    c8.cpu.pc += 2;
}

fn shr_vx(c8: &mut Chip8, x: usize, y: usize) {
    c8.cpu.registers[0xF] = c8.cpu.registers[x] & 1;
    c8.cpu.registers[x] >>= 1;
    c8.cpu.pc += 2;
}

fn subn_vx_vy(c8: &mut Chip8, x: usize, y: usize) {
    c8.cpu.registers[0xF] = if c8.cpu.registers[y] > c8.cpu.registers[x] {
        1
    } else {
        0
    };
    let result = c8.cpu.registers[y].wrapping_sub(c8.cpu.registers[x]);
    c8.cpu.registers[x] = result;
    c8.cpu.pc += 2;
}

fn shl_vx(c8: &mut Chip8, x: usize, y: usize) {
    c8.cpu.registers[0xF] = c8.cpu.registers[x] >> 7;
    c8.cpu.registers[x] <<= 1;
    c8.cpu.pc += 2;
}

fn sne_vx_vy(c8: &mut Chip8, x: usize, y: usize) {
    if c8.cpu.registers[x] != c8.cpu.registers[y] {
        c8.cpu.pc += 2
    }
    c8.cpu.pc += 2;
}

fn ld_addr(c8: &mut Chip8, nnn: usize) {
    c8.cpu.i = nnn;
    c8.cpu.pc += 2;
}

fn jp_v0_addr(c8: &mut Chip8, nnn: u16) {}

fn rnd_vx_byte(c8: &mut Chip8, x: usize, byte: u8) {
    let mut rng = rand::thread_rng();
    let rand: u8 = rng.gen();
    c8.cpu.registers[x] = rand & byte;
    c8.cpu.pc += 2;
}

fn draw_vx_vy_nibble(c8: &mut Chip8, x: usize, y: usize, nibble: u16) {
    c8.cpu.registers[0xF] = 0;
    for i in 0..nibble {
        let line = c8.memory.data[(c8.cpu.i + i as usize)];
        let y = ((c8.cpu.registers[y] as usize + i as usize) % 32) as usize;
        for j in 0..8 {
            let x = (c8.cpu.registers[x] as usize + j) % 64;
            let bit = ((line >> (7 - j)) & 0b00000001) == 1;
            if c8.screen.data[y][x] && bit {
                c8.cpu.registers[0xF] = 1;
            }
            c8.screen.data[y][x] ^= bit;
        }
    }
    c8.cpu.pc += 2;
}

fn skp_vx(c8: &mut Chip8, x: usize) {
    if c8.keys.keypad[x] == true {
        c8.cpu.pc += 2;
    }
    c8.cpu.pc += 2;
}

fn sknp_vx(c8: &mut Chip8, x: usize) {
    if c8.keys.keypad[x] == false {
        c8.cpu.pc += 2;
    }
    c8.cpu.pc += 2;
}

fn ld_vx_dt(c8: &mut Chip8, x: usize) {
    c8.cpu.registers[x] = c8.cpu.delay;
    c8.cpu.pc += 2;
}

fn ld_vx_key(c8: &mut Chip8, x: usize) {
    for (key, &pressed) in c8.keys.keypad.iter().enumerate() {
        if pressed {
            c8.cpu.registers[x] = key as u8;
            c8.cpu.pc += 2;
        }
    }
}

fn ld_dt_vx(c8: &mut Chip8, x: usize) {
    c8.cpu.delay = c8.cpu.registers[x];
    c8.cpu.pc += 2;
}

fn ld_st_vx(c8: &mut Chip8, x: usize) {
    c8.cpu.sound = c8.cpu.registers[x];
    c8.cpu.pc += 2;
}

fn add_i_vx(c8: &mut Chip8, x: usize) {
    c8.cpu.i += c8.cpu.registers[x] as usize;
    c8.cpu.pc += 2;
}

fn ld_f_vx(c8: &mut Chip8, x: usize) {
    if c8.cpu.registers[x] < 0xF {
        c8.cpu.i = (c8.cpu.registers[x] * 5) as usize;
    }
    c8.cpu.pc += 2;
}

fn ld_bcd_vx(c8: &mut Chip8, x: usize) {
    c8.memory.data[c8.cpu.i] = c8.cpu.registers[x] / 100;
    c8.memory.data[c8.cpu.i + 1] = (c8.cpu.registers[x] % 100) / 10;
    c8.memory.data[c8.cpu.i + 2] = c8.cpu.registers[x] % 10;
    c8.cpu.pc += 2;
}

fn ld_i_vx(c8: &mut Chip8, x: usize) {
    for i in 0..(x + 1) {
        c8.memory.data[c8.cpu.i + i] = c8.cpu.registers[i]
    }
    //c8.cpu.i = c8.cpu.i + x + 1;
    c8.cpu.pc += 2;
}

fn ld_vx_i(c8: &mut Chip8, x: usize) {
    for i in 0..(x + 1) {
        c8.cpu.registers[i] = c8.memory.data[c8.cpu.i + i];
    }
    //c8.cpu.i = c8.cpu.i + x + 1;
    c8.cpu.pc += 2;
}

pub struct Instruction(pub u16);
impl Instruction {
    //Returns lowest 12-bit of the instruction
    pub fn nnn(&self) -> usize {
        let Instruction(ins) = self;
        (ins & 0x0FFF) as usize
    }
    //Returns the lowest 4-bit of the instruction
    pub fn nibble(&self) -> u16 {
        let Instruction(ins) = self;
        ins & 0x000F
    }
    //Returns the lower 4 bits of the high byte of the instruction
    pub fn x(&self) -> usize {
        let Instruction(ins) = self;
        ((ins & 0x0F00) >> 8) as usize
    }
    //Returns the upper 4 bits of the low byte of the instruction
    pub fn y(&self) -> usize {
        let Instruction(ins) = self;
        ((ins & 0x00F0) >> 4) as usize
    }
    //Returns the lowest 8 bits of the instruction
    pub fn byte(&self) -> u8 {
        let Instruction(ins) = self;
        (ins & 0x00FF) as u8
    }
    pub fn op(&self) -> u16 {
        let Instruction(ins) = self;
        (ins & 0xF000) >> 12
    }
}
