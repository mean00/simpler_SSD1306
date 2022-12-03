#include "Arduino.h"
#include "ssd1306_base.h"
#include "lnI2C.h"
class OLED_lnGd32 : public  OLEDCore
{
    public:
                OLED_lnGd32(lnI2C &wire, int reset);
        void    sendCommand(uint8_t cmd);
        void    update();
        //--
        void    beginData();
    protected:
        lnI2C &_wire;
    
};


