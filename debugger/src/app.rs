
use chip8::{cpu, chip8::Chip8};
pub use eframe::egui;
pub use eframe::App;

use crate::windows::{cpu::Cpu, disasm::Disasm};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // give default for any newly added fields
pub struct Debugger {
    /// Your Chip8 instance
    #[serde(skip)] // don’t try to serialize the running machine
    c8: Chip8,

    /// Your UI windows
    windows: Windows,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct Windows {
    #[serde(skip)]
    cpu: Cpu,

    #[serde(skip)]
    disasm: Disasm,
}

impl Default for Windows {
    fn default() -> Self {
        Windows {
            cpu: Cpu::new(),
            disasm: Disasm::new(),
        }
    }
}


impl Default for Debugger {
    fn default() -> Self {
        // create a dummy ROM so we can fill c8 later in `new`
        let dummy = &mut Vec::new();
        Debugger {
            c8: Chip8::new(dummy),
            windows: Windows {
                cpu: Cpu::new(),
                disasm: Disasm::new(),
            },
        }
    }
}

impl Debugger {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, rom_file: &mut Vec<u8>) -> Self {
        // restore previous state if there is one
        let mut app: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        // initialize your Chip8 with the real ROM
        app.c8 = Chip8::new(rom_file);
        app
    }
}

impl eframe::App for Debugger {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // persist everything except the skipped fields (`c8`, `cpu`, `disasm`)
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /*fn name(&self) -> &str {
        "Chip8 Debugger"
    }*/

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Step").clicked() {
                // step needs a mutable reference to your Chip8
                cpu::step(&mut self.c8);
            }
        });

        // pass a mutable reference to your windows, and a shared ref to c8 if they only need read
        self.windows.cpu.show(ctx, &mut true, &mut self.c8);
        self.windows.disasm.show(ctx, &mut true, &mut self.c8);
    }
}