
const uint8_t initSequence[]=
{
     SSD1306_DISPLAY_OFF                    // ae
    ,SSD1306_SET_DISPLAY_CLOCK_DIV_RATIO    // d5
    ,0x80
    ,SSD1306_SET_MULTIPLEX_RATIO            // a8
    ,0x3F
    ,SSD1306_SET_DISPLAY_OFFSET             // d3
    ,0x0
    ,SSD1306_SET_START_LINE | 0x0           // 40
    ,SSD1306_CHARGE_PUMP                    // 8d
    ,0x10                       // >>--0x10
             ,0xa1 // seg remap
             ,0xc8 // scan dir reverse
    ,SSD1306_SET_COM_PINS                   // da
    ,0x12
    ,SSD1306_SET_CONTRAST_CONTROL           // 81
    ,0x9F                        // --0x9f      
    ,SSD1306_SET_PRECHARGE_PERIOD           // d9
    ,0x22                        // --0x22
    ,SSD1306_SET_VCOM_DESELECT              // db
    ,0x40
             , 0x22,0 // page addressing mode
    ,SSD1306_DEACTIVATE_SCROLL              // 2e
    ,SSD1306_DISPLAY_ALL_ON_RESUME          // a4
    ,SSD1306_NORMAL_DISPLAY                 // a6
    ,SSD1306_DISPLAY_ON                     // af
};
