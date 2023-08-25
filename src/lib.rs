use logger;
use minifb::{Window, WindowOptions};
use std::{time::{Duration, Instant}, thread};
use std::sync::Mutex;
use rayon::prelude::*;

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

pub struct Screen {
    pub size: (usize, usize),
    pub scale: usize,
    pub title: &'static str,
    window: Window,
    pub buffer: Vec<u32>,
    pub max_update_time: Duration,
    max_update_time_as_micros: u32,
    timestamp_start: Instant,
    timestamp_end: Instant,
}
impl Screen {
    pub fn new(height: usize, width: usize, scale: usize, title: &'static str, fps: usize) -> Screen {
        return Screen {
            size: (height, width),
            scale: scale,
            title: title,
            window: Window::new(
                title,
                width * scale,
                height * scale,
                WindowOptions::default(),
            ).unwrap_or_else(|e| {
                panic!("{}", e);
            }),
            buffer: vec![0xFF_000000; width * height],
            max_update_time: Duration::from_millis(1000) / fps as u32,
            max_update_time_as_micros: (Duration::from_millis(1000) / fps as u32).as_micros() as u32,
            timestamp_start: Instant::now(),
            timestamp_end: Instant::now(),
        }
    }
    pub fn update_start(&mut self) {
        self.timestamp_start = Instant::now();
    }
    pub fn update_end(&mut self) {
        self.timestamp_end = Instant::now();
    }
    pub fn wait_for_next_redraw(&mut self) {
        let elapsed = self.timestamp_start.elapsed();
        if elapsed < self.max_update_time {
            let sleep_time = self.max_update_time - elapsed;
            thread::sleep(sleep_time);
            self.window.set_title(
                format!("{} - {:>6}|{:<6}us",
                self.title, elapsed.as_micros(),
                self.max_update_time_as_micros).as_str());
        } else {
            logger::log(logger::PREFIX_WARN, format!(
                "\x1b[31mFrame took too long\x1b[0m: \x1b[33m{:>6}\x1b[0m|\x1b[33m{:<6}\x1b[0mus",
                elapsed.as_micros(),
                self.max_update_time_as_micros,
            ).as_str());
        }
    }
    pub fn is_open(&self) -> bool {
        return self.window.is_open();
    }
    pub fn redraw(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.size.1, self.size.0).unwrap();
    }

    pub fn draw_rectangle(&mut self, y: usize, x: usize, size_y: usize, size_x: usize, color: u32) {
        let buffer_mutex = Mutex::new(self.buffer.clone());
        (y..size_y + y).into_par_iter().for_each(|y| {
            let mut buffer_guard = buffer_mutex.lock().unwrap();
            for x in x..size_x + x {
                if y >= self.size.0 || x >= self.size.1 {
                    continue;
                }
                if y * self.scale >= self.size.0 * self.scale || x * self.scale >= self.size.1 * self.scale {
                    break;
                }
                let buffer_index = y * self.size.1 + x;
                buffer_guard[buffer_index] = color;
            }
        });
        self.buffer = buffer_mutex.into_inner().unwrap();
    }
    pub fn draw_sprite(&mut self, sprite: &[u32], size_y: usize, size_x: usize, pos_y: usize, pos_x: usize) {
        let buffer_width = self.size.1;
        let buffer_mutex = Mutex::new(self.buffer.clone());
        (0..size_y).into_par_iter().for_each(|y| {
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

                let mut buffer_guard = buffer_mutex.lock().unwrap();
                buffer_guard[buffer_index] = sprite[sprite_index];
            }
        });
        self.buffer = buffer_mutex.into_inner().unwrap();
    }
}
