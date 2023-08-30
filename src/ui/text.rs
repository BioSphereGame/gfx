use rusttype::{Font, Scale, point, PositionedGlyph};

pub struct Text {
    pub pos_y: u16,
    pub pos_x: u16,
    pub size_y: usize,
    pub size_x: usize,
    pub buffer: Vec<u32>,

    pub text: String,
    pub text_size: u16,
    pub text_line_margin: u8,
    pub text_char_margin: u8,
    pub text_color: u32,
    pub text_font: super::font::FontData,
}
impl Text {
    pub fn new(
        pos_y: u16,
        pos_x: u16,
        size_y: usize,
        size_x: usize,
    
        text: String,
        text_size: u16,
        text_line_margin: u8,
        text_char_margin: u8,
        text_color: u32,
        text_font: super::font::FontData,
    ) -> Text {
        return Text {
            pos_y,
            pos_x,
            size_y,
            size_x,
            buffer: vec![0x00_000000; size_y * size_x],

            text,
            text_size,
            text_line_margin,
            text_char_margin,
            text_color,
            text_font,
        };
    }

    pub fn draw(&mut self, screen: &mut crate::Screen) {
        screen.draw_sprite(&self.buffer, self.size_y, self.size_x, self.pos_y as usize, self.pos_x as usize);
    }

    pub fn render(&mut self) {
        self.buffer = vec![0x00_000000; self.size_y * self.size_x];
        self.buffer = self.render_into_buffer().clone();

        for i in 0..self.buffer.len() {
            if self.buffer[i] != self.text_color {
                self.buffer[i] = 0x00_000000;
            }
        }
    }

    pub fn render_into_buffer(&mut self) -> Vec<u32> {
        let mut pos_y_mut = self.pos_y;
        let font = Font::try_from_bytes(self.text_font.data.as_slice()).expect("Error loading font");
        let scale = Scale::uniform(self.text_size as f32);
        let v_metrics = font.v_metrics(scale);
        
        for line in self.text.lines() {
            let glyphs: Vec<PositionedGlyph> = font.layout(line, scale, point(self.pos_x as f32, pos_y_mut as f32 + v_metrics.ascent)).collect();

            let mut char_counter = 0;
            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|x, y, v| {
                        let x = x + bounding_box.min.x as u32 + self.pos_x as u32 + char_counter as u32 * self.text_char_margin as u32;
                        let y = y + bounding_box.min.y as u32 + pos_y_mut as u32;
                        if x >= self.size_x as u32 || y >= self.size_y as u32 {
                            return;
                        }
                        let index = y * self.size_x as u32 + x;
                        if v > 0.01 {
                            if self.buffer.len() < index as usize {
                                logger::error(file!(), line!(), column!(), format!("Index out of bounds: {} > {}", index, self.buffer.len()).as_str());
                            }
                            self.buffer[index as usize] = self.text_color;
                        } else {
                            self.buffer[index as usize] = 0x00_000000;
                        }
                    });
                }
                char_counter += 1;
            }

            pos_y_mut += self.text_size / 2 + self.text_line_margin as u16;
        }

        return self.buffer.clone();
    }
}
