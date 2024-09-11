use crate::chip8::Chip8;
use speedy2d::Window;
use speedy2d::window::UserEventSender;

pub mod chip8;
pub mod renderer;


pub fn run(chip8: Chip8,
           operations_per_second: u32,
           window: Window,
           user_event_sender: UserEventSender<()>) {
    let renderer = renderer::Renderer {
        chip8,
        operations_per_second,
        last_frame_time: std::time::Instant::now(),
        last_time_tick_time: std::time::Instant::now(),
        last_instruction_time: std::time::Instant::now(),
        user_event_sender,
    };
    window.run_loop(renderer);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}