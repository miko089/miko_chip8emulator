use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;
use speedy2d::shape::Rectangle;
use speedy2d::window::{KeyScancode, UserEventSender, VirtualKeyCode, WindowHandler, WindowHelper, WindowStartupInfo};
use crate::chip8::Chip8;

pub struct Renderer {
    pub chip8: Chip8,
    pub operations_per_second: u32,
    pub last_frame_time: std::time::Instant,
    pub last_time_tick_time: std::time::Instant,
    pub last_instruction_time: std::time::Instant,
    pub user_event_sender: UserEventSender<()>,
}

impl Renderer {
    fn draw_frame(chip8: &mut Chip8, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let frame = chip8.get_screen();
        let pixel_size: usize;
        {
            let screen_size = helper.get_size_pixels();
            let x = screen_size.x;
            let y = screen_size.y;
            let max_pixel_size_x = (x / 64) as usize;
            let max_pixel_size_y = (y / 32) as usize;
            pixel_size = max_pixel_size_x.min(max_pixel_size_y);
        }
        graphics.clear_screen(Color::BLACK);
        for y in 0..32 {
            for x in 0..64 {
                if frame[y * 64 + x] == 0 {
                    continue
                }
                let rect = Rectangle::new(
                    Vector2::new(
                        (x * pixel_size) as f32,
                        (y * pixel_size) as f32,
                    ),
                    Vector2::new(
                        ((x + 1) * pixel_size) as f32,
                        ((y + 1) * pixel_size) as f32
                    )
                );
                graphics.draw_rectangle(rect, Color::WHITE);
            }
        }
    }
}

impl WindowHandler for Renderer {
    fn on_start(&mut self, _helper: &mut WindowHelper<()>, _info: WindowStartupInfo) {
        let user_event_sender = self.user_event_sender.clone();
        std::thread::spawn(move || {
            loop {
                user_event_sender
                    .send_event(())
                    .unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1) / 60);
            }
        });
    }

    fn on_user_event(&mut self, helper: &mut WindowHelper<()>, _user_event: ()) {
        helper.request_redraw();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let now = std::time::Instant::now();
        let time_since_last_frame = now.duration_since(self.last_frame_time);
        let time_since_last_instruction = now.duration_since(self.last_instruction_time);
        let time_since_last_time_tick = now.duration_since(self.last_time_tick_time);
        let time_per_frame = std::time::Duration::from_secs(1) / 60;
        let time_per_instruction =
            std::time::Duration::from_secs(1) / self.operations_per_second;
        let time_per_time_tick = std::time::Duration::from_secs(1) / 60;
        if time_since_last_frame >= time_per_frame {
            self.last_frame_time = now;
            Renderer::draw_frame(&mut self.chip8, helper, graphics);
        }

        if time_since_last_instruction >= time_per_instruction {
            self.last_instruction_time = now;
            self.chip8.next_instruction();
        }

        if time_since_last_time_tick >= time_per_time_tick {
            self.last_time_tick_time = now;
            self.chip8.timer_tick();
        }
    }

    fn on_key_down(&mut self,
                   _helper: &mut WindowHelper<()>,
                   virtual_key_code: Option<VirtualKeyCode>,
                   _scancode: KeyScancode) {
        let key_code;
        if let Some(key) = virtual_key_code {
            key_code = key;
        } else {
            return;
        }
        match key_code {
            VirtualKeyCode::Key1 => self.chip8.keyboard[0x1] = true,
            VirtualKeyCode::Key2 => self.chip8.keyboard[0x2] = true,
            VirtualKeyCode::Key3 => self.chip8.keyboard[0x3] = true,
            VirtualKeyCode::Key4 => self.chip8.keyboard[0xC] = true,
            VirtualKeyCode::Q    => self.chip8.keyboard[0x4] = true,
            VirtualKeyCode::W    => self.chip8.keyboard[0x5] = true,
            VirtualKeyCode::E    => self.chip8.keyboard[0x6] = true,
            VirtualKeyCode::R    => self.chip8.keyboard[0xD] = true,
            VirtualKeyCode::A    => self.chip8.keyboard[0x7] = true,
            VirtualKeyCode::S    => self.chip8.keyboard[0x8] = true,
            VirtualKeyCode::D    => self.chip8.keyboard[0x9] = true,
            VirtualKeyCode::F    => self.chip8.keyboard[0xE] = true,
            VirtualKeyCode::Z    => self.chip8.keyboard[0xA] = true,
            VirtualKeyCode::X    => self.chip8.keyboard[0x0] = true,
            VirtualKeyCode::C    => self.chip8.keyboard[0xB] = true,
            VirtualKeyCode::V    => self.chip8.keyboard[0xF] = true,
            _ => ()
        }
    }
    fn on_key_up(&mut self,
                 _helper: &mut WindowHelper<()>,
                 virtual_key_code: Option<VirtualKeyCode>,
                 _scancode: KeyScancode) {
        let key_code;
        if let Some(key) = virtual_key_code {
            key_code = key;
        } else {
            return;
        }
        match key_code {
            VirtualKeyCode::Key1 => self.chip8.keyboard[0x1] = false,
            VirtualKeyCode::Key2 => self.chip8.keyboard[0x2] = false,
            VirtualKeyCode::Key3 => self.chip8.keyboard[0x3] = false,
            VirtualKeyCode::Key4 => self.chip8.keyboard[0xC] = false,
            VirtualKeyCode::Q => self.chip8.keyboard[0x4] = false,
            VirtualKeyCode::W => self.chip8.keyboard[0x5] = false,
            VirtualKeyCode::E => self.chip8.keyboard[0x6] = false,
            VirtualKeyCode::R => self.chip8.keyboard[0xD] = false,
            VirtualKeyCode::A => self.chip8.keyboard[0x7] = false,
            VirtualKeyCode::S => self.chip8.keyboard[0x8] = false,
            VirtualKeyCode::D => self.chip8.keyboard[0x9] = false,
            VirtualKeyCode::F => self.chip8.keyboard[0xE] = false,
            VirtualKeyCode::Z => self.chip8.keyboard[0xA] = false,
            VirtualKeyCode::X => self.chip8.keyboard[0x0] = false,
            VirtualKeyCode::C => self.chip8.keyboard[0xB] = false,
            VirtualKeyCode::V => self.chip8.keyboard[0xF] = false,
            _ => ()
        }
    }
}