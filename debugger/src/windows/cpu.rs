use eframe::egui;
use chip8::chip8::Chip8;
pub struct Cpu;


impl Cpu{

    pub fn new() -> Cpu{
        Self{}
    }


    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool, c: &mut Chip8){
        egui::Window::new("CPU").resizable(true).open(open).show(ctx, |ui| {
            ui.horizontal(|ui|{
                ui.label("Registers");
            });
            ui.horizontal(|ui|{
                ui.label("PC");
                ui.label(format!("0x{:03X}" ,c.cpu.pc));
                ui.separator();

                ui.label("I");
                ui.label(format!("0x{:03X}" ,c.cpu.i));
                ui.separator();

                ui.label("DT");
                ui.label(format!("0x{:02X}" ,c.cpu.delay));
                ui.separator();

                ui.label("ST");
                ui.label(format!("0x{:02X}" ,c.cpu.sound));
                ui.separator();

                ui.label("SP");
                ui.label(format!("0x{:02X}" ,c.cpu.sp));
                ui.separator();



            });
        //First Row of Registers
        ui.horizontal(|ui|{
            for i in 0..8 {
                ui.label(reg_index_to_string(i));
                ui.label(format!("0x{:03X}", c.cpu.registers[i]));
                ui.separator();
            
            }
        });
        //Second Row of Registers
        ui.horizontal(|ui|{
            for i in 8..c.cpu.registers.len() {
                ui.label(reg_index_to_string(i));
                ui.label(format!("0x{:03X}", c.cpu.registers[i]));
                ui.separator();
            
            }
            
            
        });

        //TODO Stack?
        
        
        });
    }
}

pub fn reg_index_to_string(index: usize) ->String{
    format!("V{:X}",index)
}