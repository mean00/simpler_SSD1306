
#include "ssd1306_base.h"

static const uint8_t initSequence[]=
{
     SSD1306_DISPLAY_OFF
    ,SSD1306_SET_DISPLAY_CLOCK_DIV_RATIO
    ,0x80
    ,SSD1306_SET_MULTIPLEX_RATIO
    ,0x3F
    ,SSD1306_SET_DISPLAY_OFFSET
    ,0x0
    ,SSD1306_SET_START_LINE | 0x0
    ,SSD1306_CHARGE_PUMP
    ,0x14                       // >>--0x10
    ,SSD1306_MEMORY_ADDR_MODE
    ,0x00
    ,SSD1306_SET_SEGMENT_REMAP | 0x1
    ,SSD1306_COM_SCAN_DIR_DEC
    
    ,SSD1306_SET_COM_PINS
    ,0x12
    ,SSD1306_SET_CONTRAST_CONTROL
    ,0xCF                        // --0x9f
    ,SSD1306_SET_PRECHARGE_PERIOD
    ,0xF1                        // --0x22
    ,SSD1306_SET_VCOM_DESELECT
    ,0x40
    ,SSD1306_DISPLAY_ALL_ON_RESUME
    ,SSD1306_NORMAL_DISPLAY
    ,SSD1306_DISPLAY_ON 
};

const OLED_InitStruct oled_init1=
{
    sizeof(initSequence),
    initSequence
};