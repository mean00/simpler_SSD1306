/**
 * This is the longan nano gd32vf103 implementation
 * @param wire
 * @param reset
 */

#include "ssd1306_i2c_lnGd32vf103.h"
#include "ssd1306_cmd.h"
/**
 * 
 * @param wire
 * @param reset
 */
 OLED_lnGd32::OLED_lnGd32(lnI2C &wire, int reset): OLEDCore(reset),_wire(wire)
{
    
}
/**
 * 
 * @param cmd
 */
void    OLED_lnGd32::sendCommand(uint8_t cmd)
{        
    uint8_t cmd2[2]={SSD1306_COMMAND,cmd};    
    _wire.write(SSD1306_ADDR, 2,cmd2);
    
}

/**
 * 
 * @param cmd
 */
const uint8_t beginHeader[]={
    SSD1306_COMMAND,SSD1306_SET_COLUMN_ADDR,0,127,SSD1306_SET_PAGE_ADDR,0,7,SSD1306_DATA
};
void    OLED_lnGd32::beginData()
{    
      _wire.write(SSD1306_ADDR, sizeof(beginHeader),(uint8_t *)beginHeader);
}
/**
 * 
 */
void    OLED_lnGd32::update()
{
    beginData();
    uint8_t intermediary[1]={SSD1306_DATA_CONTINUE};
    const uint32_t lens[2]={1,1024};
    const uint8_t *datas[2]={intermediary,scrbuf};    
    _wire.multiWrite(SSD1306_ADDR,2,lens,datas);
}

