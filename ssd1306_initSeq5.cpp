
#define EXTVCC 0

const uint8_t initSequence[]=
{
     SSD1306_DISPLAY_OFF                    // ae
    ,SSD1306_SET_DISPLAY_CLOCK_DIV_RATIO    // d5
    ,0x80
    ,SSD1306_SET_MULTIPLEX_RATIO            // a8
    ,0x3F                                   // height -1
    ,SSD1306_SET_DISPLAY_OFFSET             // d3
    ,0x0
    ,SSD1306_SET_START_LINE | 0x0           // 40
    ,SSD1306_CHARGE_PUMP                    // 8d
    ,0x14-4* EXTVCC                         // >>--0x10
    ,SSD1306_MEMORY_ADDR_MODE               // 20
    ,0x00
    ,SSD1306_SET_SEGMENT_REMAP | 0x1        // a0
    ,SSD1306_COM_SCAN_DIR_DEC               // c8
    ,SSD1306_SET_COM_PINS                   // da
    ,0x12  // 2 for 32 lines display
    ,SSD1306_SET_CONTRAST_CONTROL           // 81
    ,0xcF*(1-EXTVCC) +EXTVCC*0x9f           // --0x9f      
    ,SSD1306_SET_PRECHARGE_PERIOD           // d9
    ,0xf1*(1-EXTVCC) +EXTVCC*0x22           // --0x22
    ,SSD1306_SET_VCOM_DESELECT              // db
    ,0x40
    ,SSD1306_DEACTIVATE_SCROLL              // 2e
    ,SSD1306_DISPLAY_ALL_ON_RESUME          // a4
    ,SSD1306_NORMAL_DISPLAY                 // a6
    ,SSD1306_DISPLAY_ON                     // af
     
    , 0x21
    ,0,127 // width -1
    ,0x22,0,7 // 32 lines = 3, 64 lines = 7
};
