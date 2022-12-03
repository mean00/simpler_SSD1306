#include "Arduino.h"
#include "Wire.h"
#include "ssd1306_base.h"

class OLED_arduino : public  OLEDCore
{
    public:
                OLED_arduino(TwoWire &wire, int address, int reset);
        void    sendCommand(uint8_t cmd);
        void    update();
        //--
        void    beginData();
    protected:
        TwoWire &_wire;
        int      _address;
    
};


