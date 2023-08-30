use rusttype::{Font, Scale, point, PositionedGlyph};

pub struct Text {
    pub pos_y: usize,
    pub pos_x: usize,
    pub size_y: usize,
    pub size_x: usize,
    pub buffer: Vec<u32>,

    pub text: String,
    pub text_size: usize,
    pub text_line_margin: usize,
    pub text_char_margin: usize,
    pub text_color: u32,
    pub text_font: super::font::FontData,
}
impl Text {
    pub fn new(
        pos_y: usize,
        pos_x: usize,
        size_y: usize,
        size_x: usize,
    
        text: String,
        text_size: usize,
        text_line_margin: usize,
        text_char_margin: usize,
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

    pub fn render(&mut self) {
        self.buffer = self.render_into_buffer().clone();
    }

    pub fn draw(&mut self, screen: &mut crate::Screen) {
        screen.draw_sprite(&self.buffer, self.size_y, self.size_x, self.pos_y, self.pos_x);
    }

    pub fn render_into_buffer(&mut self) -> Vec<u32> {
        let font = Font::try_from_bytes(self.text_font.data.as_slice()).expect("Error loading font");
        let scale = Scale::uniform(self.text_size as f32);
        let v_metrics = font.v_metrics(scale);
        let mut pos_y = self.pos_y;
        self.buffer = vec![0x00_000000; self.size_y * self.size_x];
        for line in self.text.lines() {
            let glyphs: Vec<PositionedGlyph> = font.layout(line, scale, point(self.pos_x as f32, pos_y as f32 + v_metrics.ascent)).collect();
            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|x, y, v| {
                        let x = x as usize + bounding_box.min.x as usize + self.pos_x + self.text_char_margin;
                        let y = y as usize + bounding_box.min.y as usize + pos_y;
                        let index: usize = y * self.size_x + x;
                        if v > 0.01 {
                            if self.buffer.len() < index {
                                logger::error(file!(), line!(), column!(), format!("Index out of bounds: {} > {}", index, self.buffer.len()).as_str());
                            }
                            self.buffer[index] = self.text_color;
                        } else {
                            self.buffer[index] = 0x00_000000;
                        }
                    });
                }
            }
            pos_y += self.text_size / 2 + self.text_line_margin;
        }
        return self.buffer.clone();
    }
}
