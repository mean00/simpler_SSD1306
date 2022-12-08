#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
use macroquad::prelude::*;
extern crate ssd1306;

use ssd1306::ssd1306::SSD1306;
use ssd1306::access::SSD1306Access as SSD1306Access;


mod testFont;

use crate::testFont::OpenSans_Bold9pt7b;


const SCREEN_WIDTH: usize = 128;
const SCREEN_HEIGHT: usize = 64;

struct quadAccess 
{    
    width  : usize,
    height : usize,
    white  : Color,
    black  : Color,
}
//----------------------
impl quadAccess  
{
    fn new() -> quadAccess
    {
        quadAccess{
            width : SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            white : Color::new(1.,1.,1.,1.0),
            black : Color::new(0.,0.,0.,1.0),
        }
    }
    fn flush(&mut self)
    {
        
    }
}
//-------
impl SSD1306Access for quadAccess 
{
    //-----------------------------
    fn  send_command(&mut self, command : u8)    
    {

    }
    //-----------------------------    
    fn  screen_update(&mut self, width : usize, height : usize, data : &[u8])
    {
        let zoom = 4;
        for page in 0..8
        {
            for seg in 0..128
            {
                let u=data[page*128+seg];    
                for r in 0..8
                {
                    let pix = u & (1<<(r as u32));
                    draw_rectangle((zoom*(seg)) as f32, (zoom*(page*8+r)) as f32, zoom as f32,zoom as f32,match pix
                    {
                            0 => self.black ,
                            _ => self.white,
                    });   
                }
            }
        }
    }
    //-----------------------------
    fn  reset(&mut self )
    {
        self.flush();
    }
    
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

    let mut access = quadAccess::new();

    let mut ssd  = SSD1306::new (SCREEN_WIDTH,SCREEN_HEIGHT, &mut access,
            &OpenSans_Bold9pt7b,
            &OpenSans_Bold9pt7b,
            &OpenSans_Bold9pt7b    
            );
    
    let init_seq : [u8;0] = [0;0];

    ssd.begin(&init_seq);

    ssd.fill_screen(false);

    ssd.update();
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

    //
    ssd.draw_rectangle(20,20,64,40,true);
    //
   

    //
    ssd.draw_circle(60,40,10,true);

    //ssd.print(24,24,"!!!#",true);
    ssd.print(44,24,"Hey!",true);
    ssd.print(74,48,"Hey!",false);

    ssd.draw_filled_rectangle(10,20,44,20,true);
    ssd.draw_filled_rectangle(20,24,24,8,false);

    ssd.update();

    next_frame().await;
    }
    std::println!("Exiting....");
}
