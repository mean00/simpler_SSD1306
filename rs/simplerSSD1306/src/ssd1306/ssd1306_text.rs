use super :: SSD1306;
use crate::glyph::{PFXglyph,FontInfo};
use super ::FontFamily;

//--

impl <'a>SSD1306<'a>
{   
    ///
    pub fn select_font(&mut self, f: FontFamily )
    {
        
        self.current_font_index= f;
    }
    /// 
    /// 
    pub fn check_font(& mut self )
    {
        for _i in 0..3
        {
            let info : &mut FontInfo=&mut self.font_infos[_i];
            if info.font.shrinked!=0
            {
                if info.font.hs_conf !=0x74
                {
                    panic!("HSCONF");
                }
            }
            let mut mW : usize =0;
            let mut mH : isize =0;
                    
            for i in info.font.glyphs
            {
                let x : usize = i.x_advance as usize;
                let y : isize =(-(i.y_offset as isize)) as isize;
                if x>mW {mW=x;}
                if y>mH {mH=y;}
            }
            info.max_height=(mH as usize) + 1;
            info.max_width=mW;    
        }
    }
    ///
    /// 
    /// 
    /*
    fn string_length(&mut self, text : &str) -> usize
    {
        let mut width : usize =0;
        for c in text.chars()
        {
            width+=match c
            {
                '\n' | '\r' => 0,
                x => {
                    let first = self.current_font().font.first as usize;
                    let x = x as usize;
                    if (x < first) || (x > (self.current_font().font.last as usize))
                    {
                        return 0;
                    }
                    return self.current_font().font.glyphs[(c as usize)-(first as usize)].x_advance as usize;                    
                },
            }
        }
        width
    }
   */
    ///
    /// 
    /// 
    pub fn print(&mut self, x : usize, y : usize, text  : &str)
    {
        self.cursor_x=x;
        self.cursor_y=y;
        for c in text.chars()
        {
            self.write_char(c);
            self.cursor_x +=1;
        }
    }   
    pub fn write_char(&mut self,c : char)    
    {
    }
}
// EOF
