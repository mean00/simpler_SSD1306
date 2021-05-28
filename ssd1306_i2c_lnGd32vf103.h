#include "Arduino.h"
#include "ssd1306_base.h"
#include "lnI2C.h"
class OLED_lnGd32 : public  OLEDCore
{
    public:
                OLED_lnGd32(lnTwoWire &wire, int reset);
        void    sendCommand(uint8_t cmd);
        void    update();
        //--
        void    beginData();
    protected:
        lnTwoWire &_wire;
    
};


