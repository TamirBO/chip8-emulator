use chip8::keys::Keys;
use sdl2::keyboard::{KeyboardState, Scancode};




    pub fn handle_input(keys: &mut Keys, keyboard_state: KeyboardState) {
        keys.set_key(0x1, keyboard_state.is_scancode_pressed(Scancode::Num1));
        keys.set_key(0x2, keyboard_state.is_scancode_pressed(Scancode::Num2));
        keys.set_key(0x3, keyboard_state.is_scancode_pressed(Scancode::Num3));
        keys.set_key(0xC, keyboard_state.is_scancode_pressed(Scancode::Num4));
        keys.set_key(0x4, keyboard_state.is_scancode_pressed(Scancode::Q));
        keys.set_key(0x5, keyboard_state.is_scancode_pressed(Scancode::W));
        keys.set_key(0x6, keyboard_state.is_scancode_pressed(Scancode::E));
        keys.set_key(0xD, keyboard_state.is_scancode_pressed(Scancode::R));
        keys.set_key(0x7, keyboard_state.is_scancode_pressed(Scancode::A));
        keys.set_key(0x8, keyboard_state.is_scancode_pressed(Scancode::S));
        keys.set_key(0x9, keyboard_state.is_scancode_pressed(Scancode::D));
        keys.set_key(0xE, keyboard_state.is_scancode_pressed(Scancode::F));
        keys.set_key(0xA, keyboard_state.is_scancode_pressed(Scancode::Z));
        keys.set_key(0x0, keyboard_state.is_scancode_pressed(Scancode::X));
        keys.set_key(0xB, keyboard_state.is_scancode_pressed(Scancode::C));
        keys.set_key(0xF, keyboard_state.is_scancode_pressed(Scancode::V));

    }