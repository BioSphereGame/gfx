use super::{text::RendererText, font::FontData};

pub struct TextRendererButton {
    pub text: RendererText,
    pub pos_y: usize,
    pub pos_x: usize,
    pub size_y: usize,
    pub size_x: usize,
    pub color: u32,
    pub hover_color: u32,
    pub unclickable_color: u32,
    pub border_color: u32,
    pub border_size: usize,
    pub buffer: Vec<u32>,
    pub pause: u16,
    pub pause_state: u16,

    pub clicked: bool,
    pub be_hoverable: bool,
    pub hover: bool,
}
impl TextRendererButton {
    pub fn new(
        pos_y: usize,
        pos_x: usize,
        size_y: usize,
        size_x: usize,
        color: u32,
        hover_color: u32,
        unclickable_color: u32,
        border_color: u32,
        border_size: usize,

        font: FontData,
        pause: u16,
        text: String,
        text_size: usize,
        text_color: u32,
    ) -> TextRendererButton {
        return TextRendererButton {
            text: RendererText::new(
                (size_y / 4 - text_size / 4) as u16, 
                (size_y / 4 - text_size / 4) as u16,
                text_size as u16, 1, 1,
                text_color,
                font, text,
                size_y, size_x,
            ),
            pos_y,
            pos_x,
            size_y,
            size_x,
            color,
            hover_color,
            unclickable_color,
            border_color,
            border_size,
            buffer: vec![0x00_000000; (size_y * size_x) as usize],
            pause,
            pause_state: 0,
            clicked: false,
            be_hoverable: true,
            hover: false,
        }
    }

    pub fn draw(&mut self, screen: &mut crate::Screen) {
        screen.draw_sprite(&self.buffer, self.size_y, self.size_x, self.pos_y as usize, self.pos_x as usize);
    }

    pub fn update(&mut self, screen: &mut crate::Screen) {
        let hovereble: bool;
        if self.pause_state > 0 {
            self.pause_state -= 1;
            hovereble = false;
        } else {
            hovereble = true;
        }
        let mouse_y = screen.get_mouse_pos().1 as usize;
        let mouse_x = screen.get_mouse_pos().0 as usize;
        let hover: bool;
        
        if hovereble {
            if mouse_y >= self.pos_y && mouse_y <= self.pos_y + self.size_y && mouse_x >= self.pos_x && mouse_x <= self.pos_x + self.size_x {
                hover = true;
            } else {
                hover = false;
            }
        } else {
            hover = false;
        }

        if hovereble != self.be_hoverable {
            self.be_hoverable = hovereble;
            self.render();
        } else if hover != self.hover {
            self.hover = hover;
            self.render();
        }

        if self.hover && screen.get_mouse_keys().0 {
            self.clicked = true;
            self.pause_state = self.pause;
        } else {
            self.clicked = false;
        }
    }

    pub fn render_button(&mut self) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                if y < self.border_size || x < self.border_size || y >= self.size_y - self.border_size || x >= self.size_x - self.border_size {
                    self.buffer[y * self.size_x + x] = self.border_color;
                } else {
                    if !self.be_hoverable {
                        self.buffer[y * self.size_x + x] = self.unclickable_color;
                    } else if !self.hover {
                        self.buffer[y * self.size_x + x] = self.color;
                    } else {
                        self.buffer[y * self.size_x + x] = self.hover_color;
                    }
                }
            }
        };
    }

    pub fn render_text(&mut self) {
        self.text.render_into_buffer(&mut self.buffer);
    }

    pub fn render(&mut self) {
        self.render_button();
        self.render_text();
    }
}
