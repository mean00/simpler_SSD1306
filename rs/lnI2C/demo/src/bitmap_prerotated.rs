// Pre rotated image converter for SSD1306 
// Generated by ssd1306-tool  https://github.com/mean00/simpler_gfx 
// from rust_logo.gif 
pub const WIDTH : usize = 64;
pub const HEIGHT : usize = 64;
pub const BITMAP : [u8;512] = [
 0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0xC0,  0xC0, 
 0xC0,  0x80,  0x80,  0xF8,  0xF8,  0xF8,  0xF0,  0xE0,  0xF8,  0xFE,  0xFE,  0xFC,  0xF8,  0xF8,  0x7C,  0x3E, 
 0x3E,  0x7C,  0xF8,  0xF8,  0xFC,  0xFE,  0xFE,  0xF8,  0xE0,  0xF0,  0xF8,  0xF8,  0xF8,  0x80,  0x80,  0xC0, 
 0xC0,  0xC0,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00, 
 0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0xC0,  0xC0,  0xC0,  0xC4,  0xFE,  0xFC,  0xFC,  0xFC,  0xFF,  0xFF, 
 0x7F,  0x3F,  0x1F,  0x0F,  0x0F,  0x07,  0x07,  0x03,  0x03,  0x03,  0x01,  0x01,  0x03,  0x07,  0x0E,  0x1C, 
 0x1C,  0x0E,  0x07,  0x03,  0x01,  0x01,  0x03,  0x03,  0x03,  0x07,  0x07,  0x0F,  0x0F,  0x1F,  0x3F,  0x7F, 
 0xFF,  0xFF,  0xFC,  0xFC,  0xFC,  0xFE,  0xC4,  0xC0,  0xC0,  0xC0,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00, 
 0x00,  0x00,  0x00,  0x38,  0x78,  0xF8,  0xF9,  0xFF,  0x7F,  0x7F,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF, 
 0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF, 
 0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFE,  0xFE,  0xFE,  0xFC,  0xF8,  0xF0,  0xE0, 
 0x00,  0x01,  0x03,  0x87,  0xFF,  0xFF,  0x7F,  0x7F,  0xFF,  0xF9,  0xF8,  0x78,  0x38,  0x00,  0x00,  0x00, 
 0x00,  0x86,  0xCE,  0xFF,  0xFF,  0xFF,  0xFF,  0xFE,  0xFC,  0x1C,  0x0C,  0x0F,  0x07,  0x02,  0x00,  0x00, 
 0x00,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xF0,  0xF0,  0xF0,  0xF0,  0xF0, 
 0xF0,  0xF0,  0xF0,  0xF0,  0xF8,  0xF9,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0x7F,  0x3F,  0x1F,  0x07, 
 0x00,  0x00,  0x02,  0x07,  0x0F,  0x0C,  0x1C,  0xFC,  0xFE,  0xFF,  0xFF,  0xFF,  0xFF,  0xCE,  0x86,  0x00, 
 0x00,  0x61,  0x73,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xC0,  0x80,  0x80,  0x80,  0x80,  0x80,  0x80, 
 0x80,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0x87,  0x87,  0x87,  0x87,  0x87, 
 0x87,  0x07,  0x07,  0x07,  0x1F,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFE,  0xF8,  0xE0, 
 0xC0,  0xC0,  0xC0,  0xC0,  0xFC,  0xFC,  0xFC,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0x73,  0x61,  0x00, 
 0x00,  0x00,  0x00,  0x1C,  0x1E,  0x1F,  0x9F,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xBF,  0x3F, 
 0x3F,  0x3F,  0xBF,  0xBF,  0x3F,  0x3F,  0x3F,  0x3F,  0x3F,  0x3F,  0x3F,  0x3F,  0x3F,  0x3F,  0x3F,  0x3F, 
 0x1F,  0x00,  0x00,  0x00,  0x00,  0x01,  0x0F,  0x1F,  0x3F,  0x3F,  0x3F,  0x3F,  0xBF,  0xBF,  0x3F,  0x3F, 
 0x3F,  0xBF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0x9F,  0x1F,  0x1E,  0x1C,  0x00,  0x00,  0x00, 
 0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x03,  0x03,  0x03,  0x23,  0x7F,  0x3F,  0x3F,  0x3F,  0xFF,  0xFF, 
 0xE3,  0xE3,  0xE3,  0xF7,  0xFF,  0xFE,  0xE0,  0xC0,  0xC0,  0xC0,  0x80,  0x80,  0x80,  0x80,  0x80,  0x80, 
 0x80,  0x80,  0x80,  0x80,  0x80,  0x80,  0xC0,  0xC0,  0xC0,  0xE0,  0xFE,  0xFF,  0xF7,  0xE3,  0xE3,  0xE3, 
 0xFF,  0xFF,  0x3F,  0x3F,  0x3F,  0x7F,  0x23,  0x03,  0x03,  0x03,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00, 
 0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x03,  0x03, 
 0x03,  0x01,  0x01,  0x1F,  0x1F,  0x1F,  0x0F,  0x07,  0x1F,  0x7F,  0x7F,  0x3F,  0x1F,  0x1F,  0x3F,  0x7F, 
 0x7F,  0x3F,  0x1F,  0x1F,  0x3F,  0x7F,  0x7F,  0x1F,  0x07,  0x0F,  0x1F,  0x1F,  0x1F,  0x01,  0x01,  0x03, 
 0x03,  0x03,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00, 
];
