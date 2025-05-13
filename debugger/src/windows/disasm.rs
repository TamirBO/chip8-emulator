use eframe::{egui, egui::RichText};
use crate::windows::cpu::reg_index_to_string;
use chip8::{cpu, cpu::Instruction, chip8::Chip8, memory};
use std::{collections::VecDeque};


pub struct Disasm{
    command_list: VecDeque<String>,
}
impl Disasm {
    pub fn new() -> Disasm{
        Disasm{
            command_list: VecDeque::new(),
        }
    }

    pub fn collect(&mut self, pc: usize, c8: &Chip8){
        for i in (0..30).step_by(2){
        let ins = Instruction(c8.memory.read_word(pc+i));
        self.command_list.push_back(disassemble(&ins));

        }        

    }

    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool, c8: &mut Chip8){
        egui::Window::new("Disassembly").resizable(true).open(open).show(ctx, |ui|{
            for i in &self.command_list{
            ui.label(i);
        }
        let pc = c8.cpu.pc as usize;
        self.collect(pc, c8);
        });
}
}

















pub fn disassemble(ins: &Instruction) -> String{
    if ins.0 == 0x00E0 {
        return "CLS".to_string()
    }
    if ins.0 == 0x00EE{
        return "RET".to_string()
    }
    let op = ins.op();
    let nnn = ins.nnn();
    let nibble = ins.nibble();
    let x = ins.x();
    let vx = reg_index_to_string(x as usize);
    let y = ins.y();
    let vy = reg_index_to_string(y as usize);
    let byte = ins.byte();
        match op{
            0x0 => format!("SYS {:03X}", nnn),
            0x1 => format!("JP {:03X}", nnn),
            0x2 => format!("CALL {:03X}", nnn),
            0x3 => format!("SE {}, {:02X}", vx, byte), 
            0x4 => format!("SNE {}, {:02X}", vx, byte), 
            0x5 => format!("SE {}, {}", vx, vy), 
            0x6 => format!("LD, {}, {:02X}", vx, byte),
            0x7 => format!("ADD {}, {:02X}" ,vx, byte),
            0x8 => match ins.nibble() { //Instructions starting with 0x8 are Register Operations
                0x0 => format!("LD {}, {}", vx, vy), 
                0x1 => format!("OR {}, {}", vx, vy), 
                0x2 => format!("AND {}, {}", vx, vy), 
                0x3 => format!("XOR {}, {}", vx, vy),
                0x4 => format!("ADD {}, {}", vx, vy),
                0x5 => format!("SUB {}, {}", vx, vy),
                0x6 => format!("SHR {}", vx),
                0x7 => format!("SUBN {}, {}", vx, vy),
                0xE => format!("SHL {}", vx),
               _ => format!("Unknown")
            }
            0x9 => format!("SNE {}, {}", vx, vy),
            0xA => format!("LD I, {:03X}", nnn), 
            0xB => format!("JP V0, {:03X}", nnn),
            0xC => format!("RND {}, {:02X}", vx, byte),
            0xD => format!("DRAW {}, {}, {:X}", vx, vy, nibble),
            0xE => match ins.byte(){
                0x9E => format!("SKP, {}", vx),
                0xA1 => format!("SKNP, {}", vx),
               _ => format!("Unknown")
            }
            0xF => match ins.byte(){
                0x07 => format!("LD, {}, DT", vx),
                0x0A => format!("LD, {}, KEY", vx),
                0x15 => format!("LD DT, {}", vx),
                0x18 => format!("LD ST, {}", vx),
                0x1E => format!("ADD I, {}", vx),
                0x29 => format!("LD SPNUM, {}", vx),
                0x33 => format!("LD BCD, {}", vx),
                0x55 => format!("LD [I], {}", vx),
                0x65 => format!("LD {}, [I]", vx),
               _ => format!("Unknown")
            }
    
        

           _ => format!("Unknown")
        }
        
    }




