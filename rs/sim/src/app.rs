#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use minifb::{Key, Window, WindowOptions};
use numtoa::NumToA;

//mod bitmap_prerotated;
mod bitmap_prerotated_shrinked;
//
mod bt_logo;

extern crate ssd1306;

use ssd1306::access::SSD1306Access;
use ssd1306::ssd1306::SSD1306;

mod testFont;

use crate::testFont::OpenSans_Bold9pt7b;

const SCREEN_WIDTH: usize = 128;
const SCREEN_HEIGHT: usize = 64;
const ZOOM: usize = 4;
//const SIMPLE_BITMAP: &[u8; 512] = include_bytes!("rust_logo_compressed.h.bin");
struct miniFbAccess {
    buffer: Vec<u32>,
    window: Window,
    width: usize,
    height: usize,
    zoom: usize,
}
//----------------------
impl miniFbAccess {
    fn new(w: usize, h: usize, z: usize) -> miniFbAccess {
        let mut access = miniFbAccess {
            width: w,
            height: h,
            zoom: z,
            buffer: vec![0; w * h * h * h],
            window: Window::new(
                "Simpler SSD1306 sim -  ESC to Exit",
                w * z,
                h * z,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| panic!("{}", e)),
        };
        access.window.set_target_fps(30);
        access
    }
    fn flush(&mut self) {}
}
impl miniFbAccess {
    fn running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }
}
//-------
impl SSD1306Access for miniFbAccess {
    //-----------------------------
    fn send_command(&mut self, command: u8) {}
    //-----------------------------
    fn screen_update(
        &mut self,
        width: usize,
        height: usize,
        first_page: usize,
        nb_page: usize,
        data: &[u8],
    ) {
        for page in first_page..(first_page + nb_page) {
            for seg in 0..128 {
                let u = data[page * 128 + seg];
                for r in 0..8 {
                    let pix = u & (1 << (r as u32));
                    let x = seg;
                    let y = page * 8 + r;
                    let color = match pix {
                        0 => 0,
                        _ => 0xffff,
                    };
                    for yy in 0..self.zoom {
                        let offset = y * self.width * self.zoom * self.zoom
                            + yy * self.width * self.zoom
                            + x * self.zoom;
                        for xx in 0..self.zoom {
                            self.buffer[offset + xx] = color;
                        }
                    }
                }
            }
        }
        self.window
            .update_with_buffer(
                &self.buffer,
                self.width * self.zoom,
                self.height * self.zoom,
            )
            .unwrap();
    }
    //-----------------------------
    fn reset(&mut self) {
        self.flush();
    }
}

//---
fn main() {
    let bitmap_width = 64;
    let bitmap_height = 64;
    let access = miniFbAccess::new(SCREEN_WIDTH, SCREEN_HEIGHT, ZOOM);

    let mut ssd = SSD1306::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        Box::new(access),
        &OpenSans_Bold9pt7b,
        &OpenSans_Bold9pt7b,
        &OpenSans_Bold9pt7b,
    );
    //while access.running() {
    ssd.begin();

    ssd.fill_screen(false);

    ssd.update();
    let mut buf = [0u8; 20];
    let mut val: i32 = 1;
    loop {
        // ssd.print(36,26,"Hey!",true);

        //next_frame().await;
        //ssd.draw_line(10, 10, 120, 60, true); // \
        //next_frame().await;
        //ssd.draw_line(10, 60, 120, 10, true); // /
        //next_frame().await;
        //ssd.draw_line(10, 60, 10, 10, true); // ^ Left
        //next_frame().await;
        //ssd.draw_line(120, 10, 120, 60, true); // | right
        //next_frame().await;
        //ssd.draw_line(120, 60, 10, 60, true); // _ Bottom
        //next_frame().await;
        //ssd.draw_line(10, 10, 120, 10, true); // - top

        //

        //ssd.draw_rectangle(20, 20, 64, 40, true);
        //
        //ssd.draw_circle(60, 40, 10, true);

        //ssd.print(66, 24, "!!!#", true);
        //ssd.print(70, 26, "Hey!", true);
        //ssd.print(74, 48, "Hola", false);

        //ssd.draw_filled_rectangle(10, 20, 40, 20, true);
        //ssd.draw_filled_rectangle(20, 24, 24, 8, false);

        //ssd.draw_bitmap(0, 0, 64, 64, SIMPLE_BITMAP, false);
        //ssd.draw_bitmap_prerotated(64,0,bitmap_prerotated::WIDTH,bitmap_prerotated::HEIGHT,&bitmap_prerotated::BITMAP, true);
        ssd.draw_bitmap_prerotated_shrinked(
            64,
            0,
            bitmap_prerotated_shrinked::WIDTH,
            bitmap_prerotated_shrinked::HEIGHT,
            &bitmap_prerotated_shrinked::BITMAP_HS,
            true,
        );
        ssd.draw_bitmap_prerotated_shrinked(
            0,
            0,
            bt_logo::WIDTH,
            bt_logo::HEIGHT,
            &bt_logo::BITMAP_HS,
            true,
        );

        let s = val.numtoa_str(10, &mut buf); // base 10
        ssd.print(1, 32, s, false);
        ssd.update();
        val += 1;
    }
    //    std::println!("Exiting....");
}
