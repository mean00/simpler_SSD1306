/*
  OLED_I2C.h - Arduino/chipKit library support for 128x64 pixel SSD1306 OLEDs
  Copyright (C)2015 Rinky-Dink Electronics, Henning Karlsen. All right reserved
  
  This library has been made to make it easy to use 128x64 pixel OLED displays
  based on the SSD1306 controller chip with an Arduino or a chipKit.

  You can always find the latest version of the library at 
  http://www.RinkyDinkElectronics.com/

  This library is free software; you can redistribute it and/or
  modify it under the terms of the CC BY-NC-SA 3.0 license.
  Please see the included documents for further information.

  Commercial use of this library requires you to buy a license that
  will allow commercial use. This includes using the library,
  modified or not, as a tool to sell products.

  The license applies to all part of the library including the 
  examples and tools supplied with the library.
*/

#pragma once

#include "Arduino.h"
#include "ssd1306_cmd.h"
#include "pfxfont.h"


#define SSD1306_ADDR		0x3C

#define LEFT	0
#define RIGHT	9999
#define CENTER	9998

#define SSD1306_COMMAND			0x00
#define SSD1306_DATA			0xC0
#define SSD1306_DATA_CONTINUE           0x40

#define RST_NOT_IN_USE	255

#define fontbyte(x) cfont.font[x]  
#define bitmapbyte(x) bitmap[x]
#define bitmapdatatype unsigned char*

/**
\brief this is the sequence to initialize the screen
*/
struct OLED_InitStruct
{
  int size;
  const uint8_t *data;
};

class OLEDCore
{
public:
        enum FontSize
        {
            SmallFont,MediumFont,BigFont
        };
        class FontInfo
        {
        public:
          int               maxHeight;          
          int               maxWidth;
          const GFXfont    *font;        
        };  
        FontInfo          fontInfo[3];
        
        FontInfo          *currentFont;
        const GFXfont     *gfxFont;
        bool               inverted;
public: // extra functions
                void    drawRLEBitmap(int widthInPixel, int height, int wx, int wy, int fgcolor, int bgcolor, const uint8_t *data)    ;
                void    myDrawChar(int16_t x, int16_t y, unsigned char c,  bool invert) ;                
                void    square(int x,int y,int w, int h, bool color)
                            {
#warning this is incorrect
                                squareYInverted(x,y,w,h,color); 
                            }
                void    setFontSize(FontSize size);
                void    setFontFamily(const GFXfont *small, const GFXfont *medium, const GFXfont *big);
                void    print(int x,int y,const char *z);
                
                
                
                int     write(uint8_t c) ;
	public:
                        OLEDCore( uint8_t rst_pin);
		void	begin(OLED_InitStruct *init=NULL);
		void	setBrightness(uint8_t value);
		void	clrScr();
		void	fillScr(bool color);
		void	invert(bool mode);
		void	setPixel(uint16_t x, uint16_t y);
		void	clrPixel(uint16_t x, uint16_t y);
		void	invPixel(uint16_t x, uint16_t y);
		void	invertText(bool doInvert) {inverted=doInvert;}
		void	drawBitmap(int x, int y, uint8_t* bitmap, int sx, int sy);
		void	drawLine(int x1, int y1, int x2, int y2);
		void	clrLine(int x1, int y1, int x2, int y2);
		void	drawRect(int x1, int y1, int x2, int y2);
		void	clrRect(int x1, int y1, int x2, int y2);
		void	drawRoundRect(int x1, int y1, int x2, int y2);
		void	clrRoundRect(int x1, int y1, int x2, int y2);
		void	drawCircle(int x, int y, int radius);
		void	clrCircle(int x, int y, int radius);
       //
       virtual  void    sendCommand(uint8_t cmd)=0;
       virtual  void    update()=0;
       //
	protected:
		uint8_t			_rst_pin;

		void	drawHLine(int x, int y, int l);
		void	clrHLine(int x, int y, int l);
		void	drawVLine(int x, int y, int l);
		void	clrVLine(int x, int y, int l);
        void    squareYInverted(int x,int y,int w, int h, bool color);
                
                
        uint8_t *scrbuf;
        int     cursor_x,cursor_y;

                
};

