use std::fs;
use speedy2d::Window;
use miko_chip8emulator::{chip8, run};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let file_path_rom = "example_roms/IBM Logo.ch8";

    let bytes = fs::read(file_path_rom)?;

    let window =
        Window::new_centered("Meow", (640, 480)).unwrap();

    let user_event_sender = window.create_user_event_sender();
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(bytes);
    run(chip8, 500, window, user_event_sender);
    Ok(())
}
