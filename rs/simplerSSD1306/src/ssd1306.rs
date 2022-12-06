#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;
//
use crate::glyph::{PFXfont,FontInfo};
//
use crate::access::SSD1306Access;

mod ssd1306_gfx;
mod ssd1306_text;
//

#[derive(Copy, Clone)]
pub enum FontFamily
{
    SmallFont=0,
    MediumFont=1,
    BigFont=2,
}
//-----------
pub struct SSD1306 <'a>
{   
    width           : usize,
    height          : usize,
    access          : &'a mut dyn SSD1306Access,
    current_font_index : FontFamily,
    font_infos      : [FontInfo;3],
    cursor_x        : usize,
    cursor_y        : usize,
    invert          : bool,
    raw             : Vec::<u8>,
}
//-----------------
impl <'a>SSD1306<'a>
{
    fn current_font(&self) -> &FontInfo
    {        
        let ix = self.current_font_index as usize;
        return &self.font_infos[ix];
    }
    pub fn update(&mut self)
    {
        
        self.access.screen_update(self.width, self.height, &self.raw);
    }
    //-------------------------------------------------------------------------------
    pub fn new (w: usize, h:usize, access: &'a mut dyn SSD1306Access, 
                smallfont :  &'static PFXfont, mediumfont:  &'static PFXfont, bigfont :  &'static PFXfont                 
                ) 
                    -> SSD1306 <'a>
    {
        let fs = (w*h)>>3;
        let mut instance = SSD1306 
        {   
            width           : w,
            height          : h,
            access          : access,
            current_font_index : FontFamily::SmallFont,
            font_infos      : [ 
                                            FontInfo{max_height: 0, max_width: 0,font: smallfont},
                                            FontInfo{max_height: 0, max_width: 0,font: mediumfont}, 
                                            FontInfo{max_height: 0, max_width: 0,font: bigfont}],
            cursor_x        : 0,
            cursor_y        : 0,
            invert          : false,
            raw             : Vec::<u8>::with_capacity(fs),
        };
        unsafe { instance.raw.set_len(fs)};
        instance.check_font( );
        instance       
    }    
    //-------------------------------------------------------------------------------
    pub fn begin( &mut self,  init_sequence : &[u8])
    {
            self.access.reset();
            let n=init_sequence.len();
            for i in 0..n
            {  
                self.access.send_command(init_sequence[i]);
            }
            self.clear_screen();
    }    
}
// EOF
