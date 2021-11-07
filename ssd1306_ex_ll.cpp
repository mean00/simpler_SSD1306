#include "ssd1306_base.h"

#if 1
#define PEDANTIC(x) if(!(x)) xAssert(0);
#else
#define PEDANTIC(...) {}
#endif
void OLEDCore::setPixel(uint16_t x, uint16_t y)
{
    int by, bi;

    if ((x>=0) and (x<128) and (y>=0) and (y<64))
    {
        by=((y/8)*128)+x;
        bi=y % 8;

        scrbuf[by]=scrbuf[by] | (1<<bi);
        PEDANTIC(by>=0)
        PEDANTIC(by<1024)
    }
}

void OLEDCore::clrPixel(uint16_t x, uint16_t y)
{
    int by, bi;

    if ((x>=0) and (x<128) and (y>=0) and (y<64))
    {
        by=((y/8)*128)+x;
        bi=y % 8;

        scrbuf[by]=scrbuf[by] & ~(1<<bi);
        PEDANTIC(by>=0)
        PEDANTIC(by<1024)
        
    }
}
/**
 * 
 * @param x
 * @param y
 * @param c
 * @param color
 * @param bg
 */

void OLEDCore::square(int x,int y,int w, int h, bool color)
{
    
    if(color)
    {
        for(int xx=x;xx<x+w;xx++)
        for(int yy=y;yy<y+h;yy++)
            
            {
        	setPixel(xx,64-yy);
            }
    }else
    {
        for(int xx=x;xx<x+w;xx++)
        for(int yy=y;yy<y+h;yy++)
            
            {
        	clrPixel(xx,64-yy);
            }
        
    }
}
/**
 * 
 * @param x
 * @param y
 * @param c
 * @param invert
 */
void OLEDCore::myDrawChar(int16_t x, int16_t y, unsigned char c,  bool invert) 
{
    int cr=c;
    cr -= gfxFont->first;
    GFXglyph *glyph =  gfxFont->glyph + cr;
    uint8_t *bitmap = gfxFont->bitmap;

    int bo = glyph->bitmapOffset;
    int w = glyph->width;
    int h = glyph->height;
    int yAdvance=currentFont->maxHeight;
    int xAdvance=glyph->xAdvance+1;

    
    //
     // top & bottom
    int maxHeight=currentFont->maxHeight;
    int top=maxHeight+glyph->yOffset; 
#if 0     
    Logger("c=%c x=%d y=%d top=%d maxHeight=%d\n",c,x,y,top,maxHeight);
    if(y>=top && (y+glyph->yOffset)<64)
    {        
        square( x,
            y-top,
            xAdvance,
            top-h,
            invert);
    }
#endif
/*
    int bottom=-glyph->yOffset-h;
    if(bottom>=-2)
        square(x,y-bottom,xAdvance,bottom+2,invert);      
  */  

    //
   
    // fill left and right
    PEDANTIC(glyph->xOffset>=0)
#if 1              
    if(glyph->xOffset>0)
    {
        square( x,y+glyph->yOffset,
                glyph->xOffset,h,
                invert);    
    }
#endif    
    
#if 1        
    if(glyph->xAdvance+1 > (w+glyph->xOffset ))
    {
        square( x+w+glyph->xOffset,
                y+glyph->yOffset,
                glyph->xAdvance+1-(w+glyph->xOffset ),
                h,invert);
    }
#endif
    x+=glyph->xOffset;
    y+=glyph->yOffset;
    int dex=0;

    int bits = 0, bit = 0;
    int n=h*w;
    int mask=0;
    uint8_t *data=bitmap+bo;
    for(int hh=0;hh<h;hh++)
    {
        int ty=64-(y+hh);   
        int bi=ty%8;
        int bimask=1<<bi;
        int notbimask=~bimask;
        int start=((ty/8)*128);
        if(ty<0 || ty>128)
        {
            dex+=w/8; // this is incomplete ! should not happen though
            Logger("out of window\n");
            return;
        }
        for(int ww=0;ww<w;ww++)
        {
            if (!mask) 
            {
              bits = *data++;       
              mask=0x80;
            }
            bool set=!!   (bits & mask) ;
            set^=invert;
            
            int tx=(x+ww);
            if(tx>=0 && tx<128 )
            {
                int by=start+tx;     
                if(set)
                    scrbuf[by]|=bimask;
                else
                    scrbuf[by]&=notbimask;    
                PEDANTIC(by<1024)
                PEDANTIC(by>=0)
            }
            mask>>=1;
            dex++;
        }
    }
}