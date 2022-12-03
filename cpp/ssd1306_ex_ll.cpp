#include "ssd1306_base.h"

#if 0
#define PEDANTIC(x) if(!(x)) xAssert(0);
#else
#define PEDANTIC(...) {}
#endif

/**
 */
void OLEDCore::drawHLine(int x, int y, int l)
{  
     if (y>63 || y<0 || x<0 || (x+l)>127) return;
    
    uint8_t *p=scrbuf+ ((y/8)*128)+x; // segment + offset
    int bit=1<<(y%8);
    for(int xx=0;xx<l;xx++)
    {
        *p|=bit;
        p++;
    }    
}
/**
 */
void OLEDCore::clrHLine(int x, int y, int l)
{
    if (y>63 || y<0 || x<0 || (x+l)>127) return;
    
    uint8_t *p=scrbuf+ ((y/8)*128)+x; // segment + offset
    int bit=~(1<<(y%8));
    for(int xx=0;xx<l;xx++)
    {
        *p&=bit;
        p++;
    }    
}


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
 * 
 * This is only used by drawChar!
 */

void OLEDCore::squareYInverted(int x,int y,int w, int h, bool color)
{   
    int y3;
    if(color)
    {        
        for(int yy=y;yy<y+h;yy++)
        {
            y3=yy;
            if(y3>=64) return;
            y3=64-y3;
            uint8_t *p=scrbuf+ ((y3/8)*128)+x; // segment + offset
            int bit=1<<(y3%8);
            for(int xx=0;xx<w;xx++)
            {
                *p|=bit;
                p++;
            }
        }
    }else
    {
        for(int yy=y;yy<y+h;yy++)
        {
            y3=yy;
            if(y3>=64) return;
            y3=64-y3;
            uint8_t *p=scrbuf+ ((y3/8)*128)+x;
            int bit=~(1<<(y3%8));
            for(int xx=0;xx<w;xx++)
            {
                *p&=bit;
                p++;
            }
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

    
    // top & bottom
    
    int maxHeight=currentFont->maxHeight;
    int baseLine=glyph->yOffset;
    int topLine=glyph->yOffset+h;
    // top
    if(1 && topLine<maxHeight && y>=maxHeight)
    {
     squareYInverted(x,
            y-maxHeight  ,
             
            xAdvance,
            maxHeight-topLine,
            invert);
    }
    // baseLine is most of the time <0
    
    if(baseLine<0 && (y+baseLine)>=0)
    {
     squareYInverted(x,
            y+0*baseLine,
            xAdvance,
            -baseLine,
            invert); 
    }

    //
   
    // fill left and right
    PEDANTIC(glyph->xOffset>=0)      
    if(glyph->xOffset>0)
    {
        squareYInverted( x,y+glyph->yOffset,
                glyph->xOffset,h,
                invert);    
    }
    if(glyph->xAdvance+1 > (w+glyph->xOffset ))
    {
        squareYInverted( x+w+glyph->xOffset,
                y+glyph->yOffset,
                glyph->xAdvance+1-(w+glyph->xOffset ),
                h,invert);
    }

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
        if(ty<0 || ty>=64)
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