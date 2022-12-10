#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;
//
use crate::glyph::{PFXfont,FontInfo};
//
use crate::access::SSD1306Access;
use crate::ssd1306_init_seq1::SSD1306_INIT_SEQUENCE1;

mod ssd1306_gfx;
mod ssd1306_text;
//

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
    dirty            : [bool;8],
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
        
        let mut first : usize =0;
        let mut last :  usize =0;
        let mut found_first : bool = false;
        let mut found_last :  bool = false;
        let rnge = self.height/8;
        for i in 0..rnge
        {
            if self.dirty[i] && !found_first
            {
                first=i;
                found_first=true;
            }
            if self.dirty[rnge-i-1] && !found_last
            {
                last=rnge-i-1;
                found_last=true;
            }
        }
        if !found_first // not dirty, nothing to do
        {
            return;
        }

        self.access.screen_update(self.width, self.height, first, last+1-first, &self.raw);

        self.dirty = [false;8]
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
            dirty           : [false;8],
        };
        unsafe { instance.raw.set_len(fs)};
        instance.check_font( );
        instance       
    }    
    //-------------------------------------------------------------------------------
    pub fn begin( &mut self)
    {
          self.begin_custom(&SSD1306_INIT_SEQUENCE1);
    }    
    pub fn begin_custom( &mut self,  init_sequence : &[u8])
    {
            self.access.reset();
            let n=init_sequence.len();
            for i in 0..n
            {  
                self.access.send_command(init_sequence[i]);
            }
            self.clear_screen();
    }    
    // this does not seem to work (?)
    pub fn scan_direction(&mut self, invert : bool)
    {

        self.access.send_command( match invert
            {
                false =>  crate::ssd1306_cmd::SSD1306_COM_SCAN_DIR_INC,
                true  =>  crate::ssd1306_cmd::SSD1306_COM_SCAN_DIR_DEC,
            });
    }

}
// EOF
