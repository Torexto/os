use core::fmt;

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use font8x8::UnicodeFonts;
use spin::Mutex;
use x86_64::instructions::interrupts;

const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 8;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}

/// The framebuffer console, initialized once during kernel startup.
pub static WRITER: Mutex<Option<FrameBufferWriter>> = Mutex::new(None);

/// Install the framebuffer writer used by `fb_print!`.
pub fn init(framebuffer: &'static mut [u8], info: FrameBufferInfo) {
    let writer = FrameBufferWriter::new(framebuffer, info);
    interrupts::without_interrupts(|| {
        *WRITER.lock() = Some(writer);
    });
}

/// Print formatted text to the framebuffer console.
pub fn print(args: fmt::Arguments<'_>) {
    interrupts::without_interrupts(|| {
        if let Some(writer) = WRITER.lock().as_mut() {
            let _ = writer.write_fmt(args);
        }
    });
}

#[macro_export]
macro_rules! fb_print {
    ($($arg:tt)*) => {
        $crate::framebuffer::print(core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! fb_println {
    () => {
        $crate::fb_print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::fb_print!("{}\n", core::format_args!($($arg)*))
    };
}

impl FrameBufferWriter {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut writer = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
        };
        writer.clear(Color { r: 0, g: 0, b: 0 });
        writer
    }

    /// Wypełnia cały ekran jednym kolorem
    pub fn clear(&mut self, color: Color) {
        for y in 0..self.info.height {
            for x in 0..self.info.width {
                self.write_pixel(x, y, &color);
            }
        }
        self.x_pos = 0;
        self.y_pos = 0;
    }

    /// Zapisuje pojedynczy piksel z uwzględnieniem formatu kolorów (RGB vs BGR)
    pub fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        if x >= self.info.width || y >= self.info.height {
            return;
        }

        let pixel_offset = y * self.info.stride + x;
        let byte_offset = pixel_offset * self.info.bytes_per_pixel;

        match self.info.pixel_format {
            PixelFormat::Rgb => {
                self.framebuffer[byte_offset] = color.r;
                self.framebuffer[byte_offset + 1] = color.g;
                self.framebuffer[byte_offset + 2] = color.b;
            }
            PixelFormat::Bgr => {
                self.framebuffer[byte_offset] = color.b;
                self.framebuffer[byte_offset + 1] = color.g;
                self.framebuffer[byte_offset + 2] = color.r;
            }
            PixelFormat::U8 => {
                self.framebuffer[byte_offset] = color.r;
            }
            _ => {}
        }
    }

    /// Rysuje jeden znak na ekranie
    pub fn write_char(&mut self, c: char, color: &Color) {
        if c == '\n' {
            self.new_line();
            return;
        }

        if let Some(bitmap) = font8x8::BASIC_FONTS.get(c) {
            for (row_idx, byte) in bitmap.iter().enumerate() {
                for col_idx in 0..8 {
                    if (byte >> col_idx) & 1 == 1 {
                        self.write_pixel(self.x_pos + col_idx, self.y_pos + row_idx, color);
                    }
                }
            }
        }

        self.x_pos += FONT_WIDTH;

        // Zawijanie wiersza przy krawędzi ekranu
        if self.x_pos + FONT_WIDTH >= self.info.width {
            self.new_line();
        }
    }

    /// Wypisuje cały ciąg znaków
    pub fn write_string(&mut self, s: &str, color: &Color) {
        for c in s.chars() {
            self.write_char(c, color);
        }
    }

    fn new_line(&mut self) {
        self.x_pos = 0;
        self.y_pos += FONT_HEIGHT;

        // Jeśli wyjdziemy za ekran, wracamy na górę (w przyszłości: scrollowanie)
        if self.y_pos + FONT_HEIGHT >= self.info.height {
            self.y_pos = 0;
        }
    }
}

impl fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let white = Color {
            r: 255,
            g: 255,
            b: 255,
        };
        self.write_string(s, &white);
        Ok(())
    }
}
