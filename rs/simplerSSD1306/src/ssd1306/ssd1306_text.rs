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
    //---------------------------------------    
    pub fn print(&mut self, x : usize, y : usize, text  : &str, color : bool)
    {
        self.cursor_x=x;
        self.cursor_y=y;
        for c in text.chars()
        {
            self.write_char(c, color);
            self.cursor_x +=1;
        }
    }   
    //---------------------------------------
    fn my_square(&mut self,  x : usize, y : usize, w: usize, h:usize, color : bool)
    {
        let mut w:usize = w;
        let mut h:usize = h;

        if (w+x)>=self.width
        {    
            w=self.width-x;    
            if w<=0
            {
                 return ;
            }
        }    
        if (h+y)>=self.height
        {    
            h=self.height-y;
            if h<=0
            {
                return;
            }
        }    
        self.draw_rectangle( x, y, x+w-1, y+h-1,color);
        return ;
    }

    //---------------------------------------  
    pub fn write_char(&mut self, c: char, color : bool) -> ()
    {
    
        if  c == '\n'
        {
          self.cursor_x = 0;
          self.cursor_y +=  self.current_font().font.y_advance as usize;
          return;
        } 
        if c=='\r'
        {
          return ;
        }
        let c: usize = c as usize;
        if (c < self.current_font().font.first as usize) || (c > self.current_font().font.last as usize)
        {
            return ;
        }
        let first : usize = self.current_font().font.first as usize;
        let glyph : &PFXglyph = &self.current_font().font.glyphs[(c as usize)-first];
        let  w = glyph.width as usize;
        let  h = glyph.height as usize;
        
        // also ' ' here
        if (w <= 0) || (h <= 0)
        {
            //
            if self.cursor_y>self.current_font().max_height
            {
                self.my_square(
                        self.cursor_x,
                        self.cursor_y-self.current_font().max_height, 
                        self.current_font().font.glyphs[0].x_advance as usize,  // advance by the 1st char, not necessarily correct
                        self.current_font().max_height+(glyph.y_offset as usize),
                        color);
            }
            self.cursor_x += glyph.x_advance as usize ;    
            return ;
        }
    
        let xo = glyph.x_offset; // sic
        if (self.cursor_x +  ((xo as usize) + w)) > self.width
        {
          self.cursor_x = 0;
          self.cursor_y +=   self.current_font().font.y_advance as usize;
        }    
        
        self.cursor_x += self.my_draw_char(
                    self.cursor_x, 
                    self.cursor_y, 
                    c, color);
    }
  
    //---------------------------------------
    fn my_draw_char(&mut self,  x: usize, y : usize, c: usize, color: bool) -> usize
    { 

        let full_c = c;
        let mut c =c;
        let mut y : usize = y;
        c -= self.current_font().font.first as usize;

        let glyph : &PFXglyph = &( self.current_font().font.glyphs[c]);
                               
        let  w: usize    = glyph.width as usize;
        let  mut h: usize    = glyph.height as usize;    
        let  advv: usize = glyph.x_advance as usize +1;
        let  top : usize = (self.current_font().max_height as isize +glyph.y_offset as isize) as usize;
        // Special case
        if full_c==(' ' as usize)
        {
            if y>=top
            {
                self.my_square(x,
                            (y-top) as usize,
                            advv, //Fix!
                            self.current_font().max_height+2,!color);
            }
            return advv;
        }       
      
        
        // top & bottom
        if y > self.current_font().max_height
        {
            self.my_square(x,
                    y-self.current_font().max_height,
                    advv,
                    top,!color);
        }
        let bottom: isize =-(glyph.y_offset as isize)-(h as isize);
        if bottom>=-2
        {
            self.my_square(x,((y as isize)-bottom) as usize,advv,(bottom+2) as usize,!color);      
        }
            
        // offset is <0 most of the time
        let mut tmpy : isize = y as isize;
        tmpy+= glyph.y_offset as isize;   
        if tmpy<0
        {
            return glyph.x_advance as usize;
        }
        y = tmpy as usize;

        let left: usize =glyph.x_offset as usize;
        let mut right: isize =(advv as isize)-(w as isize + (left as isize));
        if right<0
        {
            right=0;
        } 
        let glyph_data : &[u8] = &(self.current_font().font.bitmap[ (glyph.offset as usize)..]);
        self.innerLoop(x,y, w,h,left,advv,color,glyph_data );                        
        return glyph.x_advance as usize;        
    }   
    //----------
    fn innerLoop(&mut self, x : usize, y : usize, w: usize, h : usize, left: usize, line_size: usize, color : bool, p: &'a [u8])
    {
        let mut bits : usize =0;
        let mut mask : usize =0;
        let mut col  : *mut u16;
        let mut ix : usize =0;         
        for _line in 0..h // for( int line=h-1;line>=0;line--)
        {
            // mid
            //for( int xcol=w-1;xcol>=0;xcol--)
            for _xcol in 0..w
            {
              unsafe {
                if mask==0 // reload ?
                {
                    bits= p[ix] as usize;ix+=1;
                    mask = 0x80;
                }      
                                
                if (bits & mask)!=0
                {
                    self.set_pixel(x+_xcol,y+_line,color);
                }else
                {
                    self.set_pixel(x+_xcol,y+_line,!color);
                }
              }
                mask>>=1;
            }  
        }   
    }                
  
}
// EOF