use super::font::FontData;
use rusttype::{Font, Scale, point, PositionedGlyph};

macro_rules! log_debug {
    ($s:expr) => {
        logger::log(logger::PREFIX_DEBUG, $s);
    };
}

pub struct RawText {
    pub text: String,
    pub pos_y: u16,
    pub pos_x: u16,
    pub size: u16,
    pub line_space: u8,
    pub char_space: u8,
    pub color: u32,
    pub font: FontData,
}
impl RawText {
    pub fn new(pos_y: u16, pos_x: u16, size: u16, line_space: u8, char_space: u8, color: u32, font: FontData, text: String) -> RawText {
        Self {
            text,
            pos_y,
            pos_x,
            size,
            line_space,
            char_space,
            color,
            font,
        }
    }

    pub fn draw(&mut self, screen: &mut crate::Screen) {
        let mut pos_y_mut = self.pos_y;
        let font = Font::try_from_bytes(self.font.data.as_slice()).expect("Error loading font");
        let scale = Scale::uniform(self.size as f32);
        let v_metrics = font.v_metrics(scale);
        
        for line in self.text.lines() {
            let glyphs: Vec<PositionedGlyph> = font.layout(line, scale, point(self.pos_x as f32, pos_y_mut as f32 + v_metrics.ascent)).collect();

            let mut char_counter = 0;
            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|x, y, v| {
                        let x = x + bounding_box.min.x as u32 + self.pos_x as u32 + char_counter as u32 * self.char_space as u32;
                        let y = y + bounding_box.min.y as u32 + pos_y_mut as u32;
                        if x >= screen.size.1 as u32 || y >= screen.size.0 as u32 {
                            return;
                        }
                        let index = y * screen.size.1 as u32 + x; 
                        if v > 0.01 {
                            screen.buffer[index as usize] = self.color;
                        }
                    });
                }
                char_counter += 1;
            }

            pos_y_mut += self.size / 2 + self.line_space as u16;
        }
    }
}

#[derive(Clone)]
pub struct RendererText {
    pub text: String,
    pub pos_y: u16,
    pub pos_x: u16,
    pub size: u16,
    pub line_space: u8,
    pub char_space: u8,
    pub color: u32,
    pub font: FontData,
    pub buffer: Vec<u32>,
    pub size_y: usize,
    pub size_x: usize,
}
impl RendererText {
    pub fn new(pos_y: u16, pos_x: u16, size: u16, line_space: u8, char_space: u8, color: u32, font: FontData, text: String, screen_size_y: usize, screen_size_x: usize) -> RendererText {
        Self {
            text,
            pos_y,
            pos_x,
            size,
            line_space,
            char_space,
            color,
            font,
            buffer: vec![0x00_000000; screen_size_y * screen_size_x],
            size_y: screen_size_y,
            size_x: screen_size_x,
        }
    }

    pub fn draw(&mut self, screen: &mut crate::Screen) {
        screen.draw_sprite(&self.buffer, self.size_y, self.size_x, self.pos_y as usize, self.pos_x as usize);
    }

    pub fn render(&mut self) {
        log_debug!(format!("Rendering text: `{}`.", self.text).as_str());
        let mut pos_y_mut = self.pos_y;
        let font = Font::try_from_bytes(self.font.data.as_slice()).expect("Error loading font");
        let scale = Scale::uniform(self.size as f32);
        let v_metrics = font.v_metrics(scale);
        
        for line in self.text.lines() {
            let glyphs: Vec<PositionedGlyph> = font.layout(line, scale, point(self.pos_x as f32, pos_y_mut as f32 + v_metrics.ascent)).collect();

            let mut char_counter = 0;
            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|x, y, v| {
                        let x = x + bounding_box.min.x as u32 + self.pos_x as u32 + char_counter as u32 * self.char_space as u32;
                        let y = y + bounding_box.min.y as u32 + pos_y_mut as u32;
                        if x >= self.size_x as u32 || y >= self.size_y as u32 {
                            return;
                        }
                        let index = y * self.size_x as u32 + x;
                        // For debug:
                        // println!("x: {}, y: {}, x*y: {}, v: {}, indes: {}", x, y, x * y, v, index);
                        if v > 0.01 {
                            self.buffer[index as usize] = self.color;
                        }
                    });
                }
                char_counter += 1;
            }

            pos_y_mut += self.size / 2 + self.line_space as u16;
        }

        for i in 0..self.buffer.len() {
            if self.buffer[i] != self.color {
                self.buffer[i] = 0x00_000000;
            }
        }
    }

    pub fn render_into_buffer(&mut self, buffer: &mut Vec<u32>) {
        log_debug!(format!("Rendering text: `{}` into buffer.", self.text).as_str());
        let mut pos_y_mut = self.pos_y;
        let font = Font::try_from_bytes(self.font.data.as_slice()).expect("Error loading font");
        let scale = Scale::uniform(self.size as f32);
        let v_metrics = font.v_metrics(scale);
        
        for line in self.text.lines() {
            let glyphs: Vec<PositionedGlyph> = font.layout(line, scale, point(self.pos_x as f32, pos_y_mut as f32 + v_metrics.ascent)).collect();

            let mut char_counter = 0;
            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|x, y, v| {
                        let x = x + bounding_box.min.x as u32 + self.pos_x as u32 + char_counter as u32 * self.char_space as u32;
                        let y = y + bounding_box.min.y as u32 + pos_y_mut as u32;
                        if x >= self.size_x as u32 || y >= self.size_y as u32 {
                            return;
                        }
                        let index = y * self.size_x as u32 + x;
                        if index < buffer.len() as u32 {
                            // For debug:
                            // println!("x: {}, y: {}, x*y: {}, v: {}, indes: {}", x, y, x * y, v, index);
                            if v > 0.01 {
                                buffer[index as usize] = self.color;
                            }
                        }
                    });
                }
                char_counter += 1;
            }

            pos_y_mut += self.size / 2 + self.line_space as u16;
        }
    }
}
