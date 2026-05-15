use super::FontFamily;
use super::SSD1306;
use simpler_gfx::{FontInfo, PFXglyph};

//--

impl SSD1306 {
    //
    pub fn select_font(&mut self, f: FontFamily) {
        self.current_font_index = f;
    }
    //
    //
    pub fn check_font(&mut self) {
        for _i in 0..3 {
            let info: &mut FontInfo = &mut self.font_infos[_i];
            if info.font.shrinked != 0 && info.font.hs_conf != 0x74 {
                panic!("HSCONF");
            }
            let mut mW: usize = 0;
            let mut mH: isize = 0;

            for i in info.font.glyphs {
                let x: usize = i.x_advance as usize;
                let y: isize = -(i.y_offset as isize);
                if x > mW {
                    mW = x;
                }
                if y > mH {
                    mH = y;
                }
            }
            info.max_height = (mH as usize) + 1;
            info.max_width = mW;
        }
    }

    fn string_length(&mut self, text: &str) -> usize {
        let mut width: usize = 0;
        for c in text.chars() {
            width += match c {
                '\n' | '\r' => 0,
                x => {
                    let first = self.current_font().font.first as usize;
                    let x = x as usize;
                    if (x < first) || (x > (self.current_font().font.last as usize)) {
                        return 0;
                    }
                    return self.current_font().font.glyphs[(c as usize) - first].x_advance
                        as usize;
                }
            }
        }
        width
    }
    //---------------------------------------
    pub fn print(&mut self, x: usize, y: usize, text: &str, color: bool) {
        self.cursor_x = x;
        self.cursor_y = y;
        for c in text.chars() {
            self.write_char(c, color);
            self.cursor_x += 1;
        }
    }
    //---------------------------------------
    fn my_square(&mut self, x: usize, y: isize, w: isize, h: isize, color: bool) {
        if x >= self.width {
            return;
        }

        let mut out_y = y;
        let mut out_h = h;

        if out_y < 0 {
            out_h += out_y;
            out_y = 0;
        }
        if out_h <= 0 {
            return;
        }

        if out_y >= self.height as isize {
            return;
        }

        if (out_h + out_y) >= self.height as isize {
            out_h = self.height as isize - out_y;
        }

        let mut out_w = w;
        if (out_w + x as isize) >= self.width as isize {
            out_w = self.width as isize - x as isize;
        }
        if out_w <= 0 {
            return;
        }

        self.draw_filled_rectangle(
            x,
            out_y as usize,
            (out_w - 1) as usize,
            (out_h - 1) as usize,
            color,
        );
    }

    //---------------------------------------
    pub fn write_char(&mut self, c: char, color: bool) {
        if c == '\n' {
            self.cursor_x = 0;
            self.cursor_y += self.current_font().font.y_advance as usize;
            return;
        }
        if c == '\r' {
            return;
        }
        let c: usize = c as usize;
        if (c < self.current_font().font.first as usize)
            || (c > self.current_font().font.last as usize)
        {
            return;
        }
        let first: usize = self.current_font().font.first as usize;
        let glyph: &PFXglyph = &self.current_font().font.glyphs[c - first];
        let w = glyph.width as isize;
        let h = glyph.height as isize;

        // also ' ' here
        if (w <= 0) || (h <= 0) {
            //
            if self.cursor_y > self.current_font().max_height {
                self.my_square(
                    self.cursor_x,
                    self.cursor_y as isize - self.current_font().max_height as isize,
                    self.current_font().font.glyphs[0].x_advance as isize, // advance by the 1st char, not necessarily correct
                    (self.current_font().max_height as isize) + (glyph.y_offset as isize),
                    color,
                );
            }
            self.cursor_x += glyph.x_advance as usize;
            return;
        }

        let xo = glyph.x_offset; // sic
        if (self.cursor_x as isize + ((xo as isize) + w)) > self.width as isize {
            self.cursor_x = 0;
            self.cursor_y += self.current_font().font.y_advance as usize;
        }

        self.cursor_x += self.my_draw_char(self.cursor_x, self.cursor_y, c, color);
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
    fn my_draw_char(&mut self, x: usize, y: usize, c: usize, color: bool) -> usize {
        let full_c = c;
        let mut c = c;
        let y: isize = y as isize;
        let first = self.current_font().font.first as usize;
        if c < first {
            return 0;
        }
        c -= first;

        let glyph: &PFXglyph = &(self.current_font().font.glyphs[c]);

        let w: usize = glyph.width as usize;
        let h: usize = glyph.height as usize;
        let xAdvance: usize = glyph.x_advance as usize + 1;

        let max_height: isize = self.current_font().max_height as isize;

        // baseLine is most of the time <0
        let base_line: isize = glyph.y_offset as isize;
        let top_line: isize = base_line + h as isize;
        //let bottom_line : isize = -(glyph.y_offset as isize)-(h as isize);

        //
        if full_c == (' ' as usize) {
            if y >= top_line {
                self.my_square(
                    x,
                    y - top_line,
                    xAdvance as isize, //Fix!
                    (self.current_font().max_height + 2) as isize,
                    !color,
                );
            }
            return xAdvance;
        }

        // top
        if top_line < max_height && y >= max_height {
            self.my_square(
                x,
                y - max_height,
                xAdvance as isize,
                max_height - top_line,
                !color,
            );
        }

        // bottom
        let bottom: isize = -(glyph.y_offset as isize) - (h as isize);
        if bottom >= -2 && (y + base_line) >= 0 {
            self.my_square(x, y - bottom, xAdvance as isize, bottom + 2, !color);
        }
        // left
        if glyph.x_offset > 0 {
            self.my_square(
                x,
                y + glyph.y_offset as isize,
                glyph.x_offset as isize,
                h as isize,
                !color,
            );
        }
        // right
        let right_offset: usize = w + glyph.x_offset as usize;
        let right: isize = glyph.x_advance as isize + 1 - right_offset as isize;
        if right > 0 {
            self.my_square(
                x + right_offset,
                y + glyph.y_offset as isize,
                right,
                h as isize,
                !color,
            );
        }

        // main loop
        let x = x + glyph.x_offset as usize;
        let y_start: isize = y + glyph.y_offset as isize;
        let mut ix: usize = 0;
        let mut mask: usize = 0;
        let mut bits: usize = 0;
        let p: &[u8] = &(self.current_font().font.bitmap[(glyph.offset as usize)..]);

        for line in 0..h {
            let py = y_start + line as isize;
            let valid_y = py >= 0 && py < self.height as isize;
            for xcol in 0..w {
                let px = x + xcol;
                if mask == 0 {
                    bits = p[ix] as usize;
                    ix += 1;
                    mask = 0x80;
                }
                if valid_y && px < self.width {
                    self.set_pixel(
                        px,
                        py as usize,
                        if (bits & mask) != 0 { color } else { !color },
                    );
                }
                mask >>= 1;
            }
        }
        glyph.x_advance as usize
    }
}
// EOF
