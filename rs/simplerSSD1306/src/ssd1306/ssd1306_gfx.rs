
use super::SSD1306;
use crate::ssd1306_cmd::*;

fn myAbs(a : usize, b: usize) -> usize
{
	if a>b 
	{
		return a-b;
	}
	return b-a;
}
//--------------
fn myMin(a : usize, b: usize) -> usize
{
	if a>b 
	{
		return b;
	}
	return a;
}
//--------------

impl <'a>SSD1306<'a>
{
	//-----------------------------
	pub fn set_pixel(&mut self, x: usize, y: usize, color : bool)	
	{
		let  screen_buffer =&mut self.raw;

		if   (x>=self.width) ||  (y>=self.height)
		{
			return;
		}
		
		let by=((y/8)*self.width)+x;
		let bi=y & 7;

		if color
		{
			screen_buffer[by] |= 1<<bi;
		}else
		{
			screen_buffer[by] &=!(1<<bi);
		}			
	}


	//-----------------------------
    pub fn clear_screen(&mut self)
    {
        self.fill_screen(false);
    }
	//-----------------------------	
    pub fn fill_screen(&mut self,color : bool)
    {        
        let filler = 255*(color as u8);
		for i in 	&mut self.raw
		{
			*i = filler 
		}
    }
	//-----------------------------	
    pub fn set_brightness(&mut self, value : u8)
    {
        self.access.send_command(SSD1306_SET_CONTRAST_CONTROL);
        self.access.send_command(value);
    }
	//-----------------------------	
    pub fn set_invert_mode(&mut self, invert : bool)
    {
		self.access.send_command( match invert
					{
						true  => SSD1306_INVERT_DISPLAY,
						false => SSD1306_NORMAL_DISPLAY,
					} );
    }
	//-----------------------------	
	// this is the fast one
	//------------------------------
	pub fn draw_hline(&mut self, x: usize, y:usize, length: usize, color : bool)
	{
		if x>=self.width || y >= self.height
		{
			return;
		}
		let mut end=x+length;
		if end >= self.width
		{
			end=self.width-1;
		}
		let screen_buffer =&mut self.raw;
		let bim : u8 =(1<< (y & 7)) as u8;
		let mut bym = ((y/8)*self.width)+x;
		if color
		{
			for _i in x..=end
			{				
				screen_buffer[bym] |= bim;
				bym+=1;
			}
		}else
		{
			for _i in x..=end
			{
				screen_buffer[bym] &=!bim;
				bym+=1;
			}
		}
	}
	//-----------------------------	
	pub fn draw_vline(&mut self, x: usize, y:usize, length: usize, color : bool)
	{
		if x>=self.width || y >= self.height
		{
			return;
		}
		let mut end=y+length;
		if end >= self.height
		{
			end=self.height-1;
		}
		let  screen_buffer =&mut self.raw;
		if color
		{
			for i in y..=end
			{
				let by=((i/8)*self.width)+x;
				let bi=i & 7;

				screen_buffer[by] |= 1<<bi;
			}
		}else {
			for i in y..=end
			{
				let by=((i/8)*self.width)+x;
				let bi=i & 7;

				screen_buffer[by] &=!(1<<bi);
			}
		}
	}
	//----------------------------
	pub fn draw_rectangle(&mut self, x: usize, y:usize, w : usize, h : usize, color : bool)
	{		
		self.draw_hline(x, y, w, color);
		self.draw_hline(x, y+h, w, color);

		self.draw_vline(x, y, h, color);
		self.draw_vline(x+w, y, h, color);
	}
	//----------------------------
	pub fn draw_filled_rectangle(&mut self, x: usize, y:usize, w: usize, h : usize, color : bool)
	{

		for yy in y..=(y+h)
		{
			self.draw_hline(x, yy, w, color);	
		}
	}

	//-----------------------------	
	pub fn draw_line(&mut self, x1: usize, y1:usize, x2: usize, y2: usize, color : bool)	
	{
		let mut tmp: usize ;
		let mut x1 = x1;
		let mut x2 = x2;
		let mut y1 = y1;
		let mut y2 = y2;

		if y1==y2
		{
			if x1>x2
			{
				tmp=x1;
				x1=x2;
				x2=tmp;
			}
			self.draw_hline(x1, y1, x2-x1,color);
			return;

		}
		if x1==x2
		{
			if y1>y2
			{
				tmp=y1;
				y1=y2;
				y2=tmp;
			}
			self.draw_vline(x1, y1, y2-y1,color);
			return;
		}
		
		
		let mut acc : isize=0;
		if myAbs(x2,x1)>myAbs(y2,y1)
		{
			let  mut delta : isize;
			delta = ((myAbs(y2,y1)*4096)/myAbs(x2,x1)) as isize;			
			if x1>x2
			{
				tmp=x1;
				x1=x2;
				x2=tmp;
				tmp=y1;
				y1=y2;
				y2=tmp;
			}
			if y2 < y1
			{
				delta=-delta;
			}
			let mut y1 : isize = y1 as isize;
			for i in x1..=x2
			{
				self.set_pixel(i, y1 as usize,color);
				acc+=delta;
				let inc=acc/4096;
				if inc!=0
				{
					y1+=inc;
					acc-=inc*4096;
				}
			}
			
		}
		else
		{
			let  mut delta : isize=((myAbs(x2,x1)*4096)/myAbs(y2,y1)) as isize;
			if y1>y2
			{
				tmp=x1;
				x1=x2;
				x2=tmp;
				tmp=y1;
				y1=y2;
				y2=tmp;
			}
			if x2 < x1
			{
				delta=-delta;
			}
			let mut x1 : isize = x1 as isize;
			for i in y1..=y2
			{
				self.set_pixel(x1 as usize, i,color);
				acc+=delta;
				let inc=acc/4096;
				if inc!=0
				{
					x1+=inc;
					acc-=inc*4096;
				}
			}			
		}	
	}

	//----------------------------------------------------------------------------
	
	pub fn draw_bitmap(&mut self, x: usize, y:usize, w: usize,  h : usize, data : &[u8], color : bool)	
	{
		let mut  bit :usize;		
		
		for cy in 0..h
		{
			bit= cy & 7;
			for cx in 0..w			
			{
				let byte =data[cx+((cy/8)*cx)];
				if (byte & (1<<bit))!=0
				{
					self.set_pixel(x+cx, y+cy,color);
				}
				else
				{
					self.set_pixel(x+cx, y+cy,!color);
				}
			}
		}      
	}
	//----------------------------------------------------------------------------
	pub fn draw_circle(&mut self, x: usize, y:usize, radius: usize,  color : bool)	
	{
		let mut f : isize = 1 - (radius as isize);
		let mut ddF_x : isize = 1;
		let mut ddF_y = -2 * (radius as isize);
		let mut x1 = 0;
		let mut y1 = radius;
		
		
		self.set_pixel(x, y + radius, color);
		self.set_pixel(x, y - radius, color);
		self.set_pixel(x + radius, y, color);
		self.set_pixel(x - radius, y, color);
	
		while x1 < y1
		{
			if f >= 0
			{
				y1-=1;
				ddF_y += 2;
				f+= ddF_y;
			}
			x1+=1;
			ddF_x += 2;
			f += ddF_x;    
			self.set_pixel(x + x1, y + y1, color);
			self.set_pixel(x - x1, y + y1, color);
			self.set_pixel(x + x1, y - y1, color);
			self.set_pixel(x - x1, y - y1, color);
			self.set_pixel(x + y1, y + x1, color);
			self.set_pixel(x - y1, y + x1, color);
			self.set_pixel(x + y1, y - x1, color);
			self.set_pixel(x - y1, y - x1, color);
		}
	}

}




