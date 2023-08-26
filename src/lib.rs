use logger;
use minifb::{Window, WindowOptions};
use rusttype::{Font, Scale, point, PositionedGlyph};

pub fn say_hi() {
    logger::log(
        logger::PREFIX_DEBUG,
        format!("Booting {}GFX v{}{} up...",
            logger::COLOR_BOLD_GREEN,
            env!("CARGO_PKG_VERSION"),
            logger::COLOR_RESET,
        ).as_str()
    );
}

pub struct WindowOptionsSettings {
    pub borderless: bool,
    pub title: bool,
    pub resize: bool,
    pub topmost: bool,
    pub transparency: bool,
}
impl WindowOptionsSettings {
    pub fn new(borderless: bool, title: bool, resize: bool, topmost: bool, transparency: bool) -> WindowOptionsSettings {
        return WindowOptionsSettings {
            borderless: borderless,
            title: title,
            resize: resize,
            topmost: topmost,
            transparency: transparency,
        }
    }
}

pub struct Screen {
    pub size: (usize, usize),
    pub scale: usize,
    pub title: &'static str,
    pub raw_title: &'static str,
    window: Window,
    pub buffer: Vec<u32>,
    pub max_update_time_as_micros: u128,
    pub max_update_time_as_millis: f64,
    pub font_data: Vec<u8>,
}
impl Screen {
    pub fn new(height: usize, width: usize, scale: usize, title: &'static str, fps: usize, window_options: WindowOptionsSettings, font: Vec<u8>) -> Screen {
        return Screen {
            size: (height, width),
            scale: scale,
            title: title,
            raw_title: title,
            window: Window::new(
                title,
                width * scale,
                height * scale,
                WindowOptions{
                    borderless: window_options.borderless,
                    title: window_options.title,
                    resize: window_options.resize,
                    scale: minifb::Scale::X1,
                    scale_mode: minifb::ScaleMode::AspectRatioStretch,
                    topmost: window_options.topmost,
                    transparency: window_options.transparency,
                    none: false,
                },
            ).unwrap_or_else(|e| {
                panic!("{}", e);
            }),
            buffer: vec![0xFF_000000; width * height],
            max_update_time_as_micros: 1000000 / fps as u128,
            max_update_time_as_millis: (1000000.0 / fps as f64) / 1000.0,
            font_data: font.clone(),
        }
    }
    pub fn is_open(&self) -> bool {
        return self.window.is_open();
    }
    pub fn redraw(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.size.1, self.size.0).unwrap();
    }
    pub fn add_to_title(&mut self, text: String) {
        self.window.set_title(format!("{} - {}", self.raw_title, text).as_str());
    }
    
    pub fn get_pressed_keys(&self) -> Vec<u32> {
        let keys = self.window.get_keys();
        let mut key_codes: Vec<u32> = vec!();
        for key in keys {
            key_codes.push(key as u32);
        }
        return key_codes;
    }
    pub fn get_mouse_pos(&self) -> (f32, f32) {
        return self.window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
    }
    pub fn get_mouse_keys(&self) -> (bool, bool, bool) {
        let keys = (
            self.window.get_mouse_down(minifb::MouseButton::Left),
            self.window.get_mouse_down(minifb::MouseButton::Middle),
            self.window.get_mouse_down(minifb::MouseButton::Right),
        );
        return keys;
    }


    pub fn draw_rectangle(&mut self, pos_y: usize, pos_x: usize, size_y: usize, size_x: usize, color: u32) {
        let buffer_width = self.size.1;
        for y in 0..size_y {
            let buffer_row_start = (y + pos_y) * buffer_width;

            for x in 0..size_x {
                if y >= self.size.0 || x >= self.size.1 {
                    continue;
                }
                if x + pos_x >= self.size.1 || y + pos_y >= self.size.0 {
                    break;
                }

                let buffer_index = buffer_row_start + (x + pos_x);

                self.buffer[buffer_index] = color;
            }
        };
    }
    pub fn draw_sprite(&mut self, sprite: &[u32], size_y: usize, size_x: usize, pos_y: usize, pos_x: usize) {
        let buffer_width = self.size.1;
        for y in 0..size_y {
            let buffer_row_start = (y + pos_y) * buffer_width;
            let sprite_row_start = y * size_x;

            for x in 0..size_x {
                if y >= self.size.0 || x >= self.size.1 {
                    continue;
                }
                if x + pos_x >= self.size.1 || y + pos_y >= self.size.0 {
                    break;
                }

                let buffer_index = buffer_row_start + (x + pos_x);
                let sprite_index = sprite_row_start + x;

                self.buffer[buffer_index] = sprite[sprite_index];
            }
        };
    }
    pub fn draw_text(&mut self, pos_y: usize, pos_x: usize, size: usize, color: u32, secondary_color: u32, text: &str) {
        let font = Font::try_from_bytes(self.font_data.as_slice()).expect("Error loading font");
        let scale = Scale::uniform(size as f32);
        let v_metrics = font.v_metrics(scale);
        let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, point(pos_x as f32, pos_y as f32 + v_metrics.ascent)).collect();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x + bounding_box.min.x as u32 + pos_x as u32;
                    let y = y + bounding_box.min.y as u32 + pos_y as u32;
                    let index = y * self.size.1 as u32 + x;
                    if v > 0.65 {
                        self.buffer[index as usize] = color;
                    } else if v > 0.45 {
                        self.buffer[index as usize] = secondary_color;
                    }
                });
            } else {
                logger::log(
                    logger::PREFIX_WARN,
                    format!("Character {} not found in font", text).as_str()
                );
            }
        }
    }
}
