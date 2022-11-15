/**
 * This is the arduino bluepill specific implentation
 * @param wire
 * @param reset
 */

#include "ssd1306_i2c_arduino.h"
#include "ssd1306_cmd.h"
/**
 * 
 * @param wire
 * @param reset
 */
OLED_arduino::OLED_arduino(TwoWire &wire, int address, int reset) : OLEDCore(reset),_wire(wire),_address(address)
{
    
}
/**
 * 
 * @param cmd
 */
void    OLED_arduino::sendCommand(uint8_t cmd)
{    
    _wire.beginTransmission(SSD1306_ADDR);
    _wire.write(SSD1306_COMMAND);
    _wire.write(cmd);
    _wire.endTransmission();    
}

/**
 * 
 * @param cmd
 */
const uint8_t beginHeader[]={
    SSD1306_COMMAND,SSD1306_SET_COLUMN_ADDR,0,127,SSD1306_SET_PAGE_ADDR,0,7,SSD1306_DATA
};
void    OLED_arduino::beginData()
{    
    _wire.beginTransmission(SSD1306_ADDR);
    _wire.write((uint8_t *)beginHeader,sizeof(beginHeader));
    _wire.endTransmission();   
}
/**
 * 
 */
void    OLED_arduino::update()
{
    beginData();
 #define CHUNK 16      
    for (int b=0; b<1024; b+=CHUNK)		// Send data
    {
     _wire.beginTransmission(SSD1306_ADDR);
     _wire.write(SSD1306_DATA_CONTINUE);
     _wire.write(scrbuf+b,CHUNK);
     _wire.endTransmission();
    }
    
}

