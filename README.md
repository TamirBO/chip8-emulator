# CHIP-8 Emulator

A simple, modular CHIP-8 emulator library written in Rust, with optional SDL2-based frontend for rendering and input handling.

## Features

* **Core** (`chip8.rs`): Implements the CHIP-8 virtual machine state and main emulation loop.
* **CPU** (`cpu.rs`): Decodes and executes CHIP-8 instructions.
* **Memory** (`memory.rs`): Manages the 4K memory space and ROM loading.
* **Display** (`screen.rs`): Maintains the 64×32 monochrome display buffer and draw routines.
* **Input** (`keys.rs`): Handles the 16-key hexadecimal keypad state.
* **SDL2 Frontend** (`sdl_backend/`): Uses SDL2 for graphics and input.

  * `display.rs`: Renders the framebuffer to an SDL window.
  * `input.rs`: Maps physical keyboard events to CHIP-8 keys.
  * `main.rs`: Example binary that ties together the emulator core and SDL frontend.


## Prerequisites

* Rust (stable)
* [cargo](https://doc.rust-lang.org/cargo/)
* [SDL2](https://www.libsdl.org/) development libraries

  * On Debian/Ubuntu: `sudo apt install libsdl2-dev`
  * On macOS (Homebrew): `brew install sdl2`

## Building

```bash
# Clone the repository
git clone https://github.com/TamirBO/chip8-emulator.git
cd chip8-emulator

# Build the emulator library and SDL2 backend
cargo build --release
```

## Running with SDL2 Frontend

```bash
# Run the SDL2-based emulator, specifying a CHIP-8 ROM
cargo run --release --bin sdl_backend path/to/rom.ch8
```

Controls:

| CHIP-8 Key | Physical Key |
| ---------- | ------------ |
| 1 2 3 C    | 1 2 3 4      |
| 4 5 6 D    | Q W E R      |
| 7 8 9 E    | A S D F      |
| A 0 B F    | Z X C V      |

Close the window or press `Esc` to exit.



## Project Layout

```
/src
├── chip8.rs           # Core VM state and loop
├── cpu.rs             # Instruction decoder/executor
├── memory.rs          # Memory map and ROM loader
├── screen.rs          # Display buffer and draw logic
├── keys.rs            # Keypad state handling
├── lib.rs             # Public API (re-exports and `Chip8` ctor)
├── main.rs            # Basic CLI example
└── sdl_backend/       # SDL2-based frontend example
    ├── display.rs     # SDL rendering code
    ├── input.rs       # SDL input mapping
    └── main.rs        # Entry point for SDL2 emulator
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
