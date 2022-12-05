#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
use macroquad::prelude::*;
extern crate ssd1306;

use ssd1306::ssd1306::SSD1306;
use ssd1306::access::SSD1306Access as SSD1306Access;


mod testfont;
mod testfont2C;
use crate::testfont::NotoSans_Bold20pt7b;
use crate::testfont2C::DejaVuSans20pt7b;

const SCREEN_WIDTH: u32 = 128;
const SCREEN_HEIGHT: u32 = 64;

struct quadAccess 
{    
    width : usize,
    height : usize,
}
//----------------------
impl quadAccess 
{
   
    fn flush(&mut self)
    {
        
    }
}
//-------
impl SSD1306Access for quadAccess 
{
    fn  send_command(&mut self, command : u8)    
    {

    }
    fn  screen_update(&mut self, data : &[u8])
    {
        for y in 0..64
        {
            for x in 0..(128/8)
            {                
                let u=data[(y*128/8)]+x;
                for r in 0..8
                {
                    let pix = u & (1<<(r as u32));
                    let mut color = Color::new(0.,0.,0.,1.0);
                    if pix!=0
                    {
                        color = Color::new(1.,1.,1.,1.0);
                    }
                    draw_rectangle((2*x) as f32, (2*y) as f32, 2.,2.,color);
                }
            }
        }
    }

    
    fn  reset(&mut self )
    {
        self.flush();
    }

    
/* 
    fn send_word(&mut self,  color : u16)
    {
        let   r : f32=  full_to_unit( color, 11,5);
        let   g : f32 = full_to_unit( color, 5, 6);
        let   b : f32 = full_to_unit( color, 0, 5);

        let ix= (self.x as i32)*2;
        let iy= (self.y as i32)*2;

        let color = Color::new(r,g,b,1.0);
        draw_rectangle(ix as f32, iy as f32, 2.,2.,color);
        self.next();
    }
    fn update_hw_rotation(&mut self, rotation  : usize )
    {
        self.flush();
    }
    fn set_address(&mut self,  x: usize, y : usize, w: usize, h:usize)
    {
        self.x1=x;
        self.x2=x+w-1;
        self.y1=y;
        self.y2=y+h-1;
        self.x=self.x1;
        self.y=self.y1;
        self.flush();
    }
    fn data_end(&mut self, )
    {
       
    }
    fn data_begin(&mut self, )
    {
       
    }   
    */
}

//---
#[macroquad::main("BasicShapes")]
async fn main() {
    let mut loops = 0;
    let bitmap_width = 96;
    let bitmap_height = 96;
    let bitmap = include_bytes!("test_bitmap.bin");
    loop {
    loops+=1;
    if loops > 150
    {
        break;
    }
    clear_background(macroquad::color::BLACK);

    let mut access = quadAccess{  width : 128, height : 64 };

    let mut ssd  = SSD1306::new (128,64, &mut access,
            &DejaVuSans20pt7b, //NotoSans_Bold20pt7b,
            &NotoSans_Bold20pt7b,
            &NotoSans_Bold20pt7b    
            );
    
    let init_seq : [u8;0] = [0;0];

    ssd.begin(&init_seq);

    ssd.fill_screen(true);
    //next_frame().await;
    ssd.draw_line(10,10,120,60,true); // \
    //next_frame().await;
    ssd.draw_line(10,60,120,10,true); // /
    //next_frame().await;
    ssd.draw_line(10,60,10,10,true);  // ^ Left
    //next_frame().await;
    ssd.draw_line(120,10,120, 60,true);// | right
    //next_frame().await;
    ssd.draw_line(120,60,10,60,true);// _ Bottom
    //next_frame().await;
    ssd.draw_line(10,10,120,10,true);// - top

    ssd.update();

    next_frame().await;
    }
    std::println!("Exiting....");
}
