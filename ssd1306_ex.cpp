#include "ssd1306_base.h"

/**
 * \fn checkFont
 * \brief extract max width/ max height from the font
 */
static void checkFont(const GFXfont *font, OLEDCore::FontInfo *info)
{
    int mW=0,mH=0;
    int x,y;
   for(int i=font->first;i<font->last;i++)
   {
         GFXglyph *glyph  = font->glyph+i-font->first;
         x=glyph->xAdvance;
         y=-glyph->yOffset;
         if(x>mW) mW=x;         
         if(y>mH) mH=y;
   }
    info->maxHeight=mH + 1;
    info->maxWidth=mW;    
    info->font=font;
}

void  OLEDCore::print(int x, int y,const char *z)
{
    cursor_x=x;
    cursor_y=y;
   int l=strlen(z);
   while(*z)
   {
       int inc=write(*z);
       cursor_x+=inc;
       z++;
   }
}

/**
 * 
 * @param small
 * @param medium
 * @param big
 */
void  OLEDCore::setFontFamily(const GFXfont *small, const GFXfont *medium, const GFXfont *big)
{
    checkFont(small, fontInfo+0);
    checkFont(medium,fontInfo+1);
    checkFont(big,   fontInfo+2);
}       

/**
 * 
 * @param size
 */
void  OLEDCore::setFontSize(FontSize size)
{
    switch(size)
    {
        case SmallFont :  currentFont=fontInfo+0;break;
        default:
        case MediumFont :   currentFont=fontInfo+1;break;
        case BigFont :   currentFont=fontInfo+2;break;
    }    
    gfxFont=currentFont->font;
}
 
/**
 * 
 * @param c
 * @return 
 */
int OLEDCore::write(uint8_t c) 
{

    if (c == '\n') 
    {
      cursor_x = 0;
      cursor_y +=           gfxFont->yAdvance;
      return 1;
    } 
    if(c=='\r')
      return 1;
    uint8_t first = gfxFont->first;
    if ((c < first) || (c > gfxFont->last)) 
        return 1;
    
    GFXglyph *glyph = gfxFont->glyph + c-first;
    int w = glyph->width,   h = glyph->height;
    if ((w <= 0) || (h <= 0)) 
    {
        cursor_x += glyph->xAdvance ;    
        return 1;
    }

    int xo = glyph->xOffset; // sic
    if ( ((cursor_x +  (xo + w)) > 128)) 
    {
      cursor_x = 0;
      cursor_y +=   gfxFont->yAdvance;
    }
    myDrawChar(cursor_x, cursor_y, c, inverted); 
    cursor_x += glyph->xAdvance ;    
    return 1;
}
/**
 * This could be optimized ** A LOT ** by swapping x & y in the source image
 * @param widthInPixel
 * @param height
 * @param wx
 * @param wy
 * @param fgcolor
 * @param bgcolor
 * @param data
 */
void OLEDCore::drawRLEBitmap(int widthInPixel, int height, int wx, int wy, int fgcolor, int bgcolor, const uint8_t *data)
{    

    bool first=true;
    int nbPixel=widthInPixel*height;    
    int mask=0;
    int cur;       
    int repeat;
    bool color;
    for( int yy=0;yy<height;yy++)    
    {
        int column=64-(wy+yy);        
        int bitToChange=1<<(column%8);
        column=128*(column/8);
        int pack=wx;
        for(int xx=0;xx<widthInPixel;)
        {
            // load next
            cur=*data++;
            if(cur==0x76)
            {
                cur=*data++;
                repeat=*data++;
            }else
            {
                repeat=1;
            }
            // 8 pixels at a time
            for(int r=0;r<repeat;r++)
            {
                int mask=0x80;
                for(int i=0;i<8;i++)
                {
                    if(mask & cur)
                    {
                        color=true;
                    }else
                        color=false;
                    mask>>=1;                
                    pack++;
                    xx++;                   
                    if(color) 
                    {
                        scrbuf[column+pack] |=bitToChange;                        
                    }
                    else 
                    {
                        scrbuf[column+pack] &=~bitToChange;                        
                    }
                    
                }
                    
            }
            
        }
    }
}
// EOF