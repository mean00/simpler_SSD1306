
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
impl <'a>SSD1306<'a>
{
	//-----------------------------
	pub fn set_pixel(&mut self, x: usize, y: usize, color : bool)	
	{
		let  screen_buffer =&mut self.raw;

		if   (x<self.width) &&  (y<self.height)
		{
			let by=((y/8)*128)+x;
			let bi=y & 7;

			
				if color
				{
					screen_buffer[by] |= 1<<bi;
				}else
				{
					screen_buffer[by] &=!(1<<bi);
				}
			
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
		for i in x..end
		{
			self.set_pixel(i, y, color);
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
		for i in y..end
		{
			self.set_pixel(x, i, color);
		}
	}
	//-----------------------------	
	pub fn draw_line(&mut self, x1: usize, y1:usize, x2: usize, y2: usize, color : bool)	
	{
		let mut tmp: usize =0;
		let mut x1 = x1;
		let mut x2 = x2;
		let mut y1 = y1;
		let mut y2 = y2;

		
		if x2<x1
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
			tmp=x1;
			x1=x2;
			x2=tmp;
			tmp=y1;
			y1=y2;
			y2=tmp;
		}
	
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
		let mut acc=0;
		if myAbs(x2,x1)>myAbs(y2,y1)
		{
			let   delta : usize;
			delta = (myAbs(y2,y1)*4096)/myAbs(x2,x1);			
			if x1>x2
			{
				//
			}
			else
			{
				for i in x1..(x2+1)
				{
					self.set_pixel(i, y1,color);
					acc+=delta;
					let inc=acc/4096;
					if inc!=0
					{
						y1+=inc;
						acc-=inc*4096;
					}
				}
			}
		}
		else
		{
			let   delta : usize;
			delta = (myAbs(x2,x1)*4096)/myAbs(y2,y1);
			if y1>y2
			{
				//---
			}
			else
			{
				for i in y1..(y2+1)				
				{
					self.set_pixel(x1, i,color);
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
	
	}

}

/*




void OLEDCore::invPixel(uint16_t x, uint16_t y)
{
    int by, bi;

    if ((x>=0) and (x<128) and (y>=0) and (y<64))
    {
        by=((y/8)*128)+x;
        bi=y % 8;

        if ((scrbuf[by] & (1<<bi))==0)
            scrbuf[by]=scrbuf[by] | (1<<bi);
        else
            scrbuf[by]=scrbuf[by] & ~(1<<bi);
    }
}


void OLEDCore::drawLine(int x1, int y1, int x2, int y2)
{
	int tmp;
	double delta, tx, ty;
	double m, b, dx, dy;
	
	if (((x2-x1)<0))
	{
		tmp=x1;
		x1=x2;
		x2=tmp;
		tmp=y1;
		y1=y2;
		y2=tmp;
	}
    if (((y2-y1)<0))
	{
		tmp=x1;
		x1=x2;
		x2=tmp;
		tmp=y1;
		y1=y2;
		y2=tmp;
	}

	if (y1==y2)
	{
		if (x1>x2)
		{
			tmp=x1;
			x1=x2;
			x2=tmp;
		}
		drawHLine(x1, y1, x2-x1);
	}
	else if (x1==x2)
	{
		if (y1>y2)
		{
			tmp=y1;
			y1=y2;
			y2=tmp;
		}
		drawVLine(x1, y1, y2-y1);
	}
	else if (myAbs(x2-x1)>myAbs(y2-y1))
	{
		delta=(double(y2-y1)/double(x2-x1));
		ty=double(y1);
		if (x1>x2)
		{
			for (int i=x1; i>=x2; i--)
			{
				setPixel(i, int(ty+0.5));
        		ty=ty-delta;
			}
		}
		else
		{
			for (int i=x1; i<=x2; i++)
			{
				setPixel(i, int(ty+0.5));
        		ty=ty+delta;
			}
		}
	}
	else
	{
		delta=(float(x2-x1)/float(y2-y1));
		tx=float(x1);
        if (y1>y2)
        {
			for (int i=y2+1; i>y1; i--)
			{
		 		setPixel(int(tx+0.5), i);
        		tx=tx+delta;
			}
        }
        else
        {
			for (int i=y1; i<y2+1; i++)
			{
		 		setPixel(int(tx+0.5), i);
        		tx=tx+delta;
			}
        }
	}

}

void OLEDCore::drawRect(int x1, int y1, int x2, int y2)
{
	int tmp;

	if (x1>x2)
	{
		tmp=x1;
		x1=x2;
		x2=tmp;
	}
	if (y1>y2)
	{
		tmp=y1;
		y1=y2;
		y2=tmp;
	}

	drawHLine(x1, y1, x2-x1);
	drawHLine(x1, y2, x2-x1);
	drawVLine(x1, y1, y2-y1);
	drawVLine(x2, y1, y2-y1+1);
}


void OLEDCore::drawRoundRect(int x1, int y1, int x2, int y2)
{
	int tmp;

	if (x1>x2)
	{
		tmp=x1;
		x1=x2;
		x2=tmp;
	}
	if (y1>y2)
	{
		tmp=y1;
		y1=y2;
		y2=tmp;
	}
	if ((x2-x1)>4 && (y2-y1)>4)
	{
		setPixel(x1+1,y1+1);
		setPixel(x2-1,y1+1);
		setPixel(x1+1,y2-1);
		setPixel(x2-1,y2-1);
		drawHLine(x1+2, y1, x2-x1-3);
		drawHLine(x1+2, y2, x2-x1-3);
		drawVLine(x1, y1+2, y2-y1-3);
		drawVLine(x2, y1+2, y2-y1-3);
	}
}

void OLEDCore::drawCircle(int x, int y, int radius)
{
	int f = 1 - radius;
	int ddF_x = 1;
	int ddF_y = -2 * radius;
	int x1 = 0;
	int y1 = radius;
	char ch, cl;
	
	setPixel(x, y + radius);
	setPixel(x, y - radius);
	setPixel(x + radius, y);
	setPixel(x - radius, y);
 
	while(x1 < y1)
	{
		if(f >= 0) 
		{
			y1--;
			ddF_y += 2;
			f += ddF_y;
		}
		x1++;
		ddF_x += 2;
		f += ddF_x;    
		setPixel(x + x1, y + y1);
		setPixel(x - x1, y + y1);
		setPixel(x + x1, y - y1);
		setPixel(x - x1, y - y1);
		setPixel(x + y1, y + x1);
		setPixel(x - y1, y + x1);
		setPixel(x + y1, y - x1);
		setPixel(x - y1, y - x1);
	}
}

void OLEDCore::drawBitmap(int x, int y, uint8_t* bitmap, int sx, int sy)
{
	int bit;
	byte data;

	for (int cy=0; cy<sy; cy++)
	{
		bit= cy % 8;
		for(int cx=0; cx<sx; cx++)
		{
			data=bitmapbyte(cx+((cy/8)*sx));
			if ((data & (1<<bit))>0)
				setPixel(x+cx, y+cy);
			else
				clrPixel(x+cx, y+cy);
		}
	}      
}
*/
