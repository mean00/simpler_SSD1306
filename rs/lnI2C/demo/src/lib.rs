#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![feature(default_alloc_error_handler)]

mod waree9;

use rnarduino as rn;

use rn::rnOsHelper::rnDelay;

use rn::rnI2C::rnI2C as rnI2C;

use ssd1306::ssd1306::SSD1306;


use lni2c_ssd1306::i2c_ssd1306;
use waree9::Waree9pt7b as fnt;

#[no_mangle]
pub extern "C" fn rust_demo()
{
    let address :u8 = lni2c_ssd1306::SSD1306_DEFAULT_I2C_ADDRESS;
    let mut i2c = rnI2C::new(0, 400*1000);
    i2c.begin(address);
    let mut access = i2c_ssd1306::new(i2c , address);
    let mut ssd = SSD1306::new (128,64,&mut access,
                            &fnt, &fnt, &fnt);  
    ssd.begin(); 

    ssd.fill_screen(false);

    ssd.update();
    let mut direction : bool = false;
    while true
    {
        ssd.clear_screen();
        ssd.draw_line(10,10,120,60,true); // \
        ssd.draw_line(10,60,120,10,true); // /
        ssd.draw_line(10,60,10,10,true);  // ^ Left
        ssd.draw_line(120,10,120, 60,true);// | right
        ssd.draw_line(120,60,10,60,true);// _ Bottom
        ssd.draw_line(10,10,120,10,true);// - top
        ssd.draw_rectangle(20,20,64,40,true);
        ssd.draw_circle(100,50,10,true);
        ssd.draw_filled_rectangle(10,20,44,20,true);
        ssd.draw_filled_rectangle(20,24,24,8,false);

        ssd.update();

        rnDelay(1000);

        ssd.clear_screen();
        ssd.print(4,24,"123456789",false);
        ssd.print(4,44,"ABCDEFHIJ",true);
        ssd.print(4,63,"#_!abcdef",true);

        rnDelay(1000);
        ssd.update();


        ssd.print(4,24,"123456789",true);
        ssd.print(4,44,"ABCDEFHIJ",true);
        ssd.print(4,63,"#_!abcdef",true);
        rnDelay(1000);
        ssd.update();

        ssd.print(4,24,"123456789",false);
        ssd.print(4,44,"ABCDEFHIJ",true);
        ssd.print(4,63,"#_!abcdef",true);
        rnDelay(1000);
        ssd.update();

        ssd.update();
        rnDelay(1000);
    }

}

// EOF
