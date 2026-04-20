#![allow(unused)]
#![allow(unused_imports)]
#![no_std]

extern crate alloc;
use alloc::boxed::Box;

mod constants;
mod i2c_ssd1306;

//
mod waree9;
mod bitmap_prerotated;
mod bitmap_prerotated_shrinked;
use ssd1306::ssd1306::SSD1306;
use waree9::Waree9pt7b as fnt;
//
use crate::i2c_ssd1306::I2cSsd106;
//
use rust_esprit::i2c;
use rust_esprit::delay_ms;
use rust_esprit::{GpioMode::lnOUTPUT, GpioMode::lnALTERNATE_OD, digital_write, pin_mode};
use rust_esprit::{lnLogger, lnLogger_init};
use constants::*;
//
lnLogger_init!();

/*
 *
 *
 */
#[unsafe(no_mangle)]
extern "C" fn user_init() {
    lnLogger!("Hello there !\n");

    pin_mode(SCL_PIN, lnALTERNATE_OD);
    pin_mode(SDA_PIN, lnALTERNATE_OD);

    rust_demo();

    lnLogger!("--end--\n");
}
/*
 *
 */
fn rust_demo()
{
    let address :u8 = i2c_ssd1306::SSD1306_DEFAULT_I2C_ADDRESS;
    let mut i2c = i2c::new(0, 400*1000);
    i2c.begin(address);
    let mut access = I2cSsd106::new(i2c , address);
    let mut ssd = SSD1306::new (128,64,Box::new(access),
                            &fnt, &fnt, &fnt);  
    ssd.begin(); 

    ssd.fill_screen(false);

    ssd.update();
    let mut direction : bool = false;
    loop 
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
        delay_ms(1000);

        ssd.clear_screen();
        ssd.print(4,24,"123456789",false);
        ssd.print(4,44,"ABCDEFHIJ",true);
        ssd.print(4,63,"#_!abcdef",true);

        ssd.update();
        delay_ms(1000);


        ssd.print(4,24,"123456789",true);
        ssd.print(4,44,"ABCDEFHIJ",true);
        ssd.print(4,63,"#_!abcdef",true);

        ssd.update();
        delay_ms(1000);

        ssd.print(4,24,"123456789",false);
        ssd.print(4,44,"ABCDEFHIJ",true);
        ssd.print(4,63,"#_!abcdef",true);

        ssd.update();
        delay_ms(1000);

        ssd.print(4,24,"123456789",false);
        ssd.print(4,44,"ABCDEFHIJ",true);
        ssd.print(4,63,"#_!abcdef",true);

        ssd.update();
        delay_ms(1000);


        ssd.draw_bitmap_prerotated(0,0,
            bitmap_prerotated::WIDTH,
            bitmap_prerotated::HEIGHT,
                        &bitmap_prerotated::BITMAP, true);
        /*
        ssd.draw_bitmap_prerotated_shrinked(64,0,
                        bitmap_prerotated_shrinked::WIDTH,
                        bitmap_prerotated_shrinked::HEIGHT,
                        &bitmap_prerotated_shrinked::BITMAP_HS, 
                        true);
    
        */ 
        ssd.update();
        delay_ms(1000);

    }

}

// EOF
// EOF
