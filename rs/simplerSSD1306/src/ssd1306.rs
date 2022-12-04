#![allow(dead_code)]
extern crate alloc;

use crate::util::unsafe_array_alloc as unsafe_array_alloc;
use crate::util::unsafe_box_allocate as unsafe_box_allocate;

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
    screen_buffer    : *mut u8,
    access          : &'a mut dyn SSD1306Access,
    current_font_index : FontFamily,
    font_infos      : [FontInfo;3],
    cursor_x        : usize,
    cursor_y        : usize,
    invert          : bool,
}
//-----------------
impl <'a>SSD1306<'a>
{
    fn current_font(&self) -> &FontInfo
    {        
        let ix = self.current_font_index as usize;
        return &self.font_infos[ix];
    }
    //-------------------------------------------------------------------------------
    pub fn new (w: usize, h:usize, access: &'a mut dyn SSD1306Access, 
                smallfont :  &'static PFXfont, mediumfont:  &'static PFXfont, bigfont :  &'static PFXfont                 
                ) 
                    -> &'a mut SSD1306 <'a>
    {
        // there is probably a better way to do this
        // we dont want to use the stack (even temporarily) as it will overflow
        unsafe {
        let  allocated :  *mut SSD1306   = unsafe_box_allocate();
        (*allocated)._init(w,h,access,smallfont,mediumfont,bigfont);        
        // We normally never free this, so a mem leak is a not a big deal            
        return &mut (*allocated);
        }
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
    //-------------------------------------------------------------------------------
    fn _init(&'a mut self,w: usize, h:usize, access: &'a mut dyn SSD1306Access, 
            smallfont :  &'static PFXfont, mediumfont:  &'static PFXfont, bigfont :  &'static PFXfont ) 
    {
        self.width              = w;
        self.height             = h;          
        self.cursor_x           = 0;   
        self.cursor_y           = 0;
        self.invert             = false;
        self.screen_buffer      = unsafe_array_alloc((w*h)/8);
        self.access             = access;
        self.font_infos[0].font = smallfont;        
        self.font_infos[1].font = mediumfont;
        self.font_infos[2].font = bigfont;                
        self.check_font( );
        self.current_font_index = FontFamily::SmallFont;
    }   
}
// EOF
