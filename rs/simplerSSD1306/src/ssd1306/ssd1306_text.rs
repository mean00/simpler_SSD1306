use super :: SSD1306;
use simpler_gfx::{PFXglyph,FontInfo};
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

        if x>=self.width
        {
            return;
        }
        if y>= self.height
        {
            return;
        }

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
        self.draw_filled_rectangle( x, y, w-1, h-1,color);
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
    /*
                                      *****  /\
                                      *****  |
                (X,Y) <--xOffset----> *****  Height
                          ^           *****  |
                        yOffset(<0)   *****  \/
                          \/          <  width >
    
    
    */
    fn my_draw_char(&mut self,  x: usize, y : usize, c: usize, color: bool) -> usize
    {        
        let full_c = c;
        let mut c =c;
        let y : isize = y as isize;
        let first = self.current_font().font.first as usize;
        if c<first
        {
            return 0;
        }
        c -= first;

        let glyph : &PFXglyph = &( self.current_font().font.glyphs[c]);
                               
        let  w: usize    = glyph.width as usize;
        let  h: usize    = glyph.height as usize;    
        let  xAdvance: usize = glyph.x_advance as usize +1;

        let max_height: isize = self.current_font().max_height as isize;

        // baseLine is most of the time <0
        let base_line : isize  = glyph.y_offset as isize;
        let top_line  : isize  = base_line + h as isize;
        //let bottom_line : isize = -(glyph.y_offset as isize)-(h as isize);

        //
        if full_c==(' ' as usize)
        {
            if y>=top_line
            {
                self.my_square(x,
                            (y-top_line) as usize,
                            xAdvance, //Fix!
                            self.current_font().max_height+2,!color);
            }
            return xAdvance;
        }       

        // top
        if  top_line<max_height && y>=max_height
        {
            self.my_square(x,
                    (y-max_height) as usize ,                    
                    xAdvance,
                    (max_height-top_line) as usize,
                    !color);
        }
        
        // bottom
        let bottom: isize =-(glyph.y_offset as isize)-(h as isize);
        if bottom >= -2 && (y+base_line)>=0
        {
            self.my_square(x,
                    (y-bottom) as usize,
                    xAdvance as usize,
                    (bottom+2) as usize,
                    !color); 
        }
        // left
        if glyph.x_offset>0
        {
            self.my_square( x,
                            (y+glyph.y_offset as isize) as usize,
                            glyph.x_offset as usize,
                                h,
                        !color);    
        }
        // right
        let right_offset : usize =  w + glyph.x_offset as usize;
        let right : isize = glyph.x_advance as isize +1 - right_offset as isize;
        if right > 0
        {            
            
            self.my_square( x+right_offset,
                    (y as isize +glyph.y_offset as isize) as usize,
                    right as usize,
                    h,
                    !color);
        }

        // main loop
        let x=x+glyph.x_offset as usize;
        let y : usize =(y+glyph.y_offset as isize) as usize;
        let mut ix : usize = 0;
        let mut mask : usize = 0;
        let mut bits : usize = 0;
        let p : &[u8] = &(self.current_font().font.bitmap[ (glyph.offset as usize)..]);

        for line in y..(y+h)
        {
            for xcol in x..(x+w)
            {
                if mask==0 // reload ?
                {
                    bits= p[ix] as usize;
                    ix+=1;
                    mask = 0x80;
                }      
                self.set_pixel(  xcol,line, match (bits & mask)!=0
                {
                    true => color,
                    false => !color,
                });
                mask>>=1;
            }  
        }        
        return glyph.x_advance as usize;
    }       
  
}
// EOF
