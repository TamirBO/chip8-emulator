use std::env;
use std::fs;
use chip8::cpu::Instruction;
use debugger::app::Debugger;
use eframe::{egui::Vec2, NativeOptions, CreationContext};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [ROM_FILE]", args[0]);
        return;
    }

    // Read ROM before launching the UI
    let mut rom_file = fs::read(&args[1])
        .expect("Failed to read ROM file");

    let native_options = NativeOptions::default();

    let _ = eframe::run_native(
        "Chip8 Debugger",           // window title
        native_options,
        Box::new(move |cc: &CreationContext<'_>| {
            // Now you can call your two-arg constructor
            Ok(Box::new(Debugger::new(cc, &mut rom_file)))
        }),
    );
}