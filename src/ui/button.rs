use super::{text::Text, font::FontData};

pub struct TextRendererButton {
    text: Text,

    pub pos_y: usize,
    pub pos_x: usize,
    pub size_y: usize,
    pub size_x: usize,

    pub base_color: u32,
    pub hovered_color: u32,
    pub pressed_color: u32,

    pub border_color: u32,
    pub border_size: usize,

    buffer: Vec<u32>,
    pub delay: u16,
    delay_counter: u16,

    pub pressed: bool,
    pub enabled: bool,
    pub hovered: bool,
}
impl TextRendererButton {
    pub fn new(
        pos_y: usize,
        pos_x: usize,
        size_y: usize,
        size_x: usize,

        text: String,
        text_size: usize,
        text_color: u32,
        font: FontData,

        base_color: u32,
        hovered_color: u32,
        pressed_color: u32,

        border_color: u32,
        border_size: usize,
        delay: u16,
    ) -> TextRendererButton {
        return TextRendererButton {
            text: Text::new(
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

            base_color,
            hovered_color,
            pressed_color,

            buffer: vec![0xFF_000000; (size_y * size_x) as usize],
            delay,
            delay_counter: 0,

            border_color,
            border_size,


            pressed: false,
            enabled: true,
            hovered: false,
        }
    }

    pub fn draw(&mut self, screen: &mut crate::Screen) {
        screen.draw_sprite(&self.buffer, self.size_y, self.size_x, self.pos_y as usize, self.pos_x as usize);
    }

    pub fn update(&mut self, screen: &mut crate::Screen) {
        let hovereble: bool;
        if self.delay_counter > 0 {
            self.delay_counter -= 1;
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

        if hovereble != self.enabled {
            self.hovered = hovereble;
            self.render();
        } else if hover != self.hovered {
            self.hovered = hover;
            self.render();
        }

        if self.hovered && screen.get_mouse_keys().0 {
            self.pressed = true;
            self.delay_counter = self.delay;
            self.hovered = false;
        } else {
            self.pressed = false;
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text.text = text;
        self.render();
    }

    pub fn render_button(&mut self) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                let index = y * self.size_x + x;
                if y <= self.border_size || y >= self.size_y - self.border_size || x <= self.border_size || x >= self.size_x - self.border_size {
                    self.buffer[index] = self.border_color;
                } else {
                    if self.enabled {
                        if self.hovered {
                            self.buffer[index] = self.hovered_color;
                        } else {
                            self.buffer[index] = self.base_color;
                        }
                    } else {
                        self.buffer[index] = self.pressed_color;
                    }
                }
            }
        };
    }

    pub fn render_text(&mut self) {
        let buffer = self.text.render_into_buffer();
        for i in 0..buffer.len() {
            if buffer[i] >> 24 != 0x00 as u32 {
                self.buffer[i] = buffer[i];
            }
        }
    }

    pub fn render(&mut self) {
        self.render_button();
        self.render_text();
    }
}
