

pub const SSD1306_COMMAND			                      :u8 = 0x00;
pub const SSD1306_DATA			                          :u8 = 0xC0;
pub const SSD1306_DATA_CONTINUE                           :u8 = 0x40;

// Fundamental Commands
pub const SSD1306_SET_CONTRAST_CONTROL					  :u8 = 0x81;
pub const SSD1306_DISPLAY_ALL_ON_RESUME					  :u8 = 0xA4;
pub const SSD1306_DISPLAY_ALL_ON						  :u8 = 0xA5;
pub const SSD1306_NORMAL_DISPLAY						  :u8 = 0xA6;
pub const SSD1306_INVERT_DISPLAY						  :u8 = 0xA7;
pub const SSD1306_DISPLAY_OFF							  :u8 = 0xAE;
pub const SSD1306_DISPLAY_ON							  :u8 = 0xAF;
pub const SSD1306_NOP									  :u8 = 0xE3;
// Scrolling Commands
pub const SSD1306_HORIZONTAL_SCROLL_RIGHT				  :u8 = 0x26;
pub const SSD1306_HORIZONTAL_SCROLL_LEFT				  :u8 = 0x27;
pub const SSD1306_HORIZONTAL_SCROLL_VERTICAL_AND_RIGHT	  :u8 = 0x29;
pub const SSD1306_HORIZONTAL_SCROLL_VERTICAL_AND_LEFT	  :u8 = 0x2A;
pub const SSD1306_DEACTIVATE_SCROLL						  :u8 = 0x2E;
pub const SSD1306_ACTIVATE_SCROLL						  :u8 = 0x2F;
pub const SSD1306_SET_VERTICAL_SCROLL_AREA				  :u8 = 0xA3;
// Addressing Setting Commands;
pub const SSD1306_SET_LOWER_COLUMN						  :u8 = 0x00;
pub const SSD1306_SET_HIGHER_COLUMN						  :u8 = 0x10;
pub const SSD1306_MEMORY_ADDR_MODE						  :u8 = 0x20;
pub const SSD1306_SET_COLUMN_ADDR						  :u8 = 0x21;
pub const SSD1306_SET_PAGE_ADDR							  :u8 = 0x22;
// Hardware Configuration Commands;
pub const SSD1306_SET_START_LINE						  :u8 = 0x40;
pub const SSD1306_SET_SEGMENT_REMAP						  :u8 = 0xA0;
pub const SSD1306_SET_MULTIPLEX_RATIO					  :u8 = 0xA8;
pub const SSD1306_COM_SCAN_DIR_INC						  :u8 = 0xC0;
pub const SSD1306_COM_SCAN_DIR_DEC						  :u8 = 0xC8;
pub const SSD1306_SET_DISPLAY_OFFSET					  :u8 = 0xD3;
pub const SSD1306_SET_COM_PINS							  :u8 = 0xDA;
pub const SSD1306_CHARGE_PUMP							  :u8 = 0x8D;
// Timing & Driving Scheme Setting Commands
pub const SSD1306_SET_DISPLAY_CLOCK_DIV_RATIO			  :u8 = 0xD5;
pub const SSD1306_SET_PRECHARGE_PERIOD					  :u8 = 0xD9;
pub const SSD1306_SET_VCOM_DESELECT						  :u8 = 0xDB;
